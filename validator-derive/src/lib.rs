use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

    let validate_stmts = match input.data {
        Data::Struct(ds) => {
            match ds.fields {
                Fields::Named(fields_named) => {
                    let mut stmts = Vec::new();
                    for field in fields_named.named.iter() {
                        if let Some(vpath) = find_validator_path(&field.attrs) {
                            let fname = field.ident.as_ref().unwrap();
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
                    stmts
                }
                Fields::Unnamed(fields_unnamed) => {
                    let mut stmts = Vec::new();
                    for (idx, field) in fields_unnamed.unnamed.iter().enumerate() {
                        if let Some(vpath) = find_validator_path(&field.attrs) {
                            let index = syn::Index::from(idx);
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
                    stmts
                }
                Fields::Unit => Vec::new(),
            }
        }
        _ => Vec::new(), // ignore enums/others for POC
    };

    let codgen = quote! {
        impl ::validator::validate::Validate for #ident {
            fn validate(&self) -> Result<(), ::validator::prelude::ValidationError> {
                #(#validate_stmts)*
                Ok(())
            }
        }
    };

    codgen.into()
}

fn find_validator_path(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        if attr.path().is_ident("validate") {
            let mut found: Option<proc_macro2::TokenStream> = None;
            let _ = attr.parse_nested_meta(|meta| {
                let p = meta.path;
                found = Some(path_to_expr_tokens(&p));
                Ok(())
            });
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

fn path_to_expr_tokens(p: &Path) -> proc_macro2::TokenStream {
    // Generate expression to create an instance for a unit struct path: e.g., Email
    // If the validator is not a unit struct, users can wrap via `With<V, T>`.
    let path_tokens = p.to_token_stream();
    quote! { #path_tokens }
}