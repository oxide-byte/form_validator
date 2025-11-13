use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Path};

/// Derive macro for `validator::validate::Validate`.
///
/// Usage:
/// ```ignore
/// use validator::Validate;
/// use validator::validators::Email;
/// use validator::validators::Positive;
///
/// #[derive(Validate)]
/// struct User {
///     #[validate(Email)]
///     email: String,
///     #[validate(Positive)]
///     age: i32,
///     // fields without `#[validate(...)]` are ignored
///     note: String,
/// }
/// ```
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate short-circuit validation from field-level #[validate(...)] annotations
    let validate_stmts = build_validate_stmts(&input.data);

    // Generate error-collecting validation from field-level #[validate(...)] annotations
    let complete_validate_stmts = build_complete_validate_stmts(&input.data);

    let guard_mod = format_ident!("__validate_guard_{}", ident);

    let codgen = quote! {
        #[allow(non_snake_case, non_camel_case_types, unused_qualifications)]
        mod #guard_mod {
            use std::cell::Cell;
            thread_local! { static DEPTH: Cell<usize> = Cell::new(0); }
            #[inline]
            pub fn enter() -> bool {
                DEPTH.with(|d| {
                    let n = d.get();
                    if n == 0 { d.set(1); true } else { false }
                })
            }
            #[inline]
            pub fn exit() {
                DEPTH.with(|d| d.set(d.get().saturating_sub(1)))
            }
        }

        impl #impl_generics ::validator::validate::Validate for #ident #ty_generics #where_clause {
            fn validate(&self) -> Result<(), ::validator::prelude::ValidationError> {
                if #guard_mod::enter() {
                    let __res: Result<(), ::validator::prelude::ValidationError> = (|| {
                        #(#validate_stmts)*
                        Ok(())
                    })();
                    #guard_mod::exit();
                    __res
                } else {
                    // Re-entrant call detected; short-circuit to avoid infinite recursion
                    Ok(())
                }
            }

            fn complete_validate(&self) -> Result<(), ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::validator::prelude::ValidationError>>> {
                if #guard_mod::enter() {
                    let __res: Result<(), ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::validator::prelude::ValidationError>>> = (|| {
                        let mut __errors: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::validator::prelude::ValidationError>> = ::std::collections::HashMap::new();
                        #(#complete_validate_stmts)*
                        if __errors.is_empty() { Ok(()) } else { Err(__errors) }
                    })();
                    #guard_mod::exit();
                    __res
                } else {
                    // Re-entrant call detected; short-circuit to avoid infinite recursion
                    Ok(())
                }
            }
        }
    };
    // For Debug purpose:
    // eprintln!("{}", codgen.to_string());
    codgen.into()
}

/// Build short-circuit validate statements for each field annotated with
/// `#[validate(...)]`. Works on the inner items of the annotation, turning
/// each validator item into a validator instance and a `validate(&field)` call.
fn build_validate_stmts(data: &Data) -> Vec<proc_macro2::TokenStream> {
    collect_field_specs(data, /*with_keys=*/ false)
        .into_iter()
        .flat_map(|spec| build_validate_for_accessor(spec.accessor, &spec.vpaths))
        .collect()
}

/// Build error-collecting validate statements for each field annotated with
/// `#[validate(...)]`. Uses the field name (or tuple index) as the error-map key
/// and iterates over each validator item inside the annotation.
fn build_complete_validate_stmts(data: &Data) -> Vec<proc_macro2::TokenStream> {
    collect_field_specs(data, /*with_keys=*/ true)
        .into_iter()
        .flat_map(|spec| {
            let key = spec.key.expect("key must be present when with_keys=true");
            build_complete_validate_for_accessor(spec.accessor, key, &spec.vpaths)
        })
        .collect()
}

/// Describes how a single field with a `#[validate(...)]` annotation should be
/// expanded:
/// - accessor: `self.field` or `self.N` for tuple structs
/// - key: optional error-map key (field name or index as string)
/// - vpaths: validator constructor expressions taken from the items of
///   the `#[validate(...)]` annotation
struct FieldSpec {
    accessor: proc_macro2::TokenStream,
    key: Option<String>,
    vpaths: Vec<proc_macro2::TokenStream>,
}

/// Collect specs for all fields that carry a `#[validate(...)]` annotation.
/// This looks only at the presence of the annotation on a field and prepares
/// accessors/keys, delegating the parsing of the items inside `( ... )` to
/// `find_validator_paths`.
fn collect_field_specs(data: &Data, with_keys: bool) -> Vec<FieldSpec> {
    let mut out = Vec::new();
    match data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(fields_named) => {
                for field in fields_named.named.iter() {
                    let vpaths = find_validator_paths(&field.attrs);
                    if vpaths.is_empty() { continue; }
                    let fname = field.ident.as_ref().expect("named field should have ident");
                    let accessor = quote! { self.#fname };
                    let key = if with_keys { Some(fname.to_string()) } else { None };
                    out.push(FieldSpec { accessor, key, vpaths });
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                for (idx, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let vpaths = find_validator_paths(&field.attrs);
                    if vpaths.is_empty() { continue; }
                    let index = syn::Index::from(idx);
                    let accessor = quote! { self.#index };
                    let key = if with_keys { Some(idx.to_string()) } else { None };
                    out.push(FieldSpec { accessor, key, vpaths });
                }
            }
            Fields::Unit => {}
        },
        _ => {}
    }
    out
}

/// Parse the inner items of a `#[validate(...)]` attribute into constructor
/// expressions for validators. For unit-like items (e.g. `Email`) we emit
/// `Email::default()`. For items with arguments (e.g. `Length(min = 3)`) we emit
/// `Length::new(min = 3)`.
fn find_validator_paths(attrs: &[Attribute]) -> Vec<proc_macro2::TokenStream> {
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("validate") { continue; }
        if let Ok(list) = attr.meta.require_list() {
            // Use syn's nested meta parser to handle items robustly (no hardcoded names)
            let _ = list.parse_nested_meta(|meta| {
                let p: Path = meta.path;
                // If the item has parentheses, capture the inner tokens and emit `path(inner)`
                if meta.input.peek(syn::token::Paren) {
                    let content;
                    let _paren = syn::parenthesized!(content in meta.input);
                    let args_tokens: proc_macro2::TokenStream = content.parse()?;
                    out.push(quote! { #p :: new ( #args_tokens ) });
                } else {
                    // Unit-like without args (e.g., Email)
                    out.push(path_to_expr_tokens(&p));
                }
                Ok(())
            });
        }
    }
    out
}

/// Helper used for unit-like validator items inside the annotation. Converts a
/// path like `Email` into `Email::default()`.
fn path_to_expr_tokens(p: &Path) -> proc_macro2::TokenStream {
    let path_tokens = p.to_token_stream();
    quote! { #path_tokens :: default() }
}

/// Emit short-circuiting validate statements for each validator item found in
/// the field's `#[validate(...)]` annotation. On first error, returns `Err`.
fn build_validate_for_accessor(
    accessor: proc_macro2::TokenStream,
    vpaths: &[proc_macro2::TokenStream],
) -> Vec<proc_macro2::TokenStream> {
    let mut stmts = Vec::new();
    for vpath in vpaths {
        let stmt = quote! {
            {
                let v = #vpath;
                if let Err(e) = v.validate(&#accessor) {
                    return Err(e);
                }
            }
        };
        stmts.push(stmt);
    }
    stmts
}

/// Emit error-collecting validate statements for each validator item found in
/// the field's `#[validate(...)]` annotation. Errors are pushed under the
/// provided key.
fn build_complete_validate_for_accessor(
    accessor: proc_macro2::TokenStream,
    key: String,
    vpaths: &[proc_macro2::TokenStream],
) -> Vec<proc_macro2::TokenStream> {
    let mut stmts = Vec::new();
    for vpath in vpaths {
        let stmt = quote! {
            {
                let v = #vpath;
                if let Err(e) = v.validate(&#accessor) {
                    __errors.entry(#key.to_string()).or_insert_with(::std::vec::Vec::new).push(e);
                }
            }
        };
        stmts.push(stmt);
    }
    stmts
}