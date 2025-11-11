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

    let validate_stmts = match &input.data {
        Data::Struct(ds) => {
            match &ds.fields {
                Fields::Named(fields_named) => {
                    let mut stmts = Vec::new();
                    for field in fields_named.named.iter() {
                        let vpaths = find_validator_paths(&field.attrs);
                        if !vpaths.is_empty() {
                            let fname = field.ident.as_ref().unwrap();
                            for vpath in vpaths {
                                let stmt = quote! {
                                    {
                                        let v = #vpath;
                                        if let Err(e) = v.validate(&self.#fname) {
                                            return Err(e);
                                        }
                                    }
                                };
                                stmts.push(stmt);
                            }
                        }
                    }
                    stmts
                }
                Fields::Unnamed(fields_unnamed) => {
                    let mut stmts = Vec::new();
                    for (idx, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let vpaths = find_validator_paths(&field.attrs);
                        if !vpaths.is_empty() {
                            let index = syn::Index::from(idx);
                            for vpath in vpaths {
                                let stmt = quote! {
                                    {
                                        let v = #vpath;
                                        if let Err(e) = v.validate(&self.#index) {
                                            return Err(e);
                                        }
                                    }
                                };
                                stmts.push(stmt);
                            }
                        }
                    }
                    stmts
                }
                Fields::Unit => Vec::new(),
            }
        }
        _ => Vec::new(), // ignore enums/others for POC
    };

    let complete_validate_stmts = match &input.data {
        Data::Struct(ds) => {
            match &ds.fields {
                Fields::Named(fields_named) => {
                    let mut stmts = Vec::new();
                    for field in fields_named.named.iter() {
                        let vpaths = find_validator_paths(&field.attrs);
                        if !vpaths.is_empty() {
                            let fname = field.ident.as_ref().unwrap();
                            let key_str = fname.to_string();
                            for vpath in vpaths {
                                let stmt = quote! {
                                    {
                                        let v = #vpath;
                                        if let Err(e) = v.validate(&self.#fname) {
                                            __errors.entry(#key_str.to_string()).or_insert_with(::std::vec::Vec::new).push(e);
                                        }
                                    }
                                };
                                stmts.push(stmt);
                            }
                        }
                    }
                    stmts
                }
                Fields::Unnamed(fields_unnamed) => {
                    let mut stmts = Vec::new();
                    for (idx, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let vpaths = find_validator_paths(&field.attrs);
                        if !vpaths.is_empty() {
                            let index = syn::Index::from(idx);
                            let key_str = idx.to_string();
                            for vpath in vpaths {
                                let stmt = quote! {
                                    {
                                        let v = #vpath;
                                        if let Err(e) = v.validate(&self.#index) {
                                            __errors.entry(#key_str.to_string()).or_insert_with(::std::vec::Vec::new).push(e);
                                        }
                                    }
                                };
                                stmts.push(stmt);
                            }
                        }
                    }
                    stmts
                }
                Fields::Unit => Vec::new(),
            }
        }
        _ => Vec::new(), // ignore enums/others for POC
    };

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

fn path_to_expr_tokens(p: &Path) -> proc_macro2::TokenStream {
    let path_tokens = p.to_token_stream();
    quote! { #path_tokens :: default() }
}