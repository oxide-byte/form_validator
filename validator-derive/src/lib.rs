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

    let validate_stmts = match input.data {
        Data::Struct(ds) => {
            match ds.fields {
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
            let s = list.tokens.to_string();
            // Split by top-level commas
            let mut parts: Vec<String> = Vec::new();
            let mut buf = String::new();
            let mut depth = 0i32;
            for ch in s.chars() {
                match ch {
                    '(' => { depth += 1; buf.push(ch); }
                    ')' => { depth -= 1; buf.push(ch); }
                    ',' if depth == 0 => {
                        if !buf.trim().is_empty() { parts.push(buf.trim().to_string()); }
                        buf.clear();
                    }
                    _ => buf.push(ch),
                }
            }
            if !buf.trim().is_empty() { parts.push(buf.trim().to_string()); }

            for part in parts {
                let part_trim = part.trim();
                if part_trim.is_empty() { continue; }
                if let Some(open) = part_trim.find('(') {
                    let close = part_trim.rfind(')').unwrap_or(part_trim.len()-1);
                    let name = part_trim[..open].trim();
                    let inner = if close > open { part_trim[open+1..close].trim() } else { "" };
                    match name {
                        "MaxLength" => {
                            if let Ok(limit) = inner.parse::<u32>() {
                                out.push(quote! { ::validator::validators::max_length::MaxLength::new(#limit) });
                            }
                        }
                        "MinLength" => {
                            if let Ok(limit) = inner.parse::<u32>() {
                                out.push(quote! { ::validator::validators::min_length::MinLength::new(#limit) });
                            }
                        }
                        "NotAllowedChars" => {
                            let inner_tokens: proc_macro2::TokenStream = inner.parse().unwrap_or_else(|_| proc_macro2::TokenStream::new());
                            out.push(quote! { ::validator::validators::not_allowed_chars::NotAllowedChars::new(#inner_tokens) });
                        }
                        _ => {
                            let p_tokens: proc_macro2::TokenStream = name.parse().unwrap_or_else(|_| proc_macro2::TokenStream::new());
                            let args_tokens: proc_macro2::TokenStream = inner.parse().unwrap_or_else(|_| proc_macro2::TokenStream::new());
                            out.push(quote! { #p_tokens ( #args_tokens ) });
                        }
                    }
                } else {
                    // Unit-like without args (e.g., Email)
                    if let Ok(p_tokens) = syn::parse_str::<Path>(part_trim) {
                        out.push(path_to_expr_tokens(&p_tokens));
                    }
                }
            }
            continue;
        }
        // Fallbacks for older/simple forms
        if let Ok(ts) = parse_validator_spec(attr) {
            out.push(ts);
            continue;
        }
        // Single simple meta without parentheses
        let mut found: Option<proc_macro2::TokenStream> = None;
        let _ = attr.parse_nested_meta(|meta| {
            let p = meta.path;
            found = Some(path_to_expr_tokens(&p));
            Ok(())
        });
        if let Some(ts) = found { out.push(ts); }
    }
    out
}

fn find_validator_path(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        if attr.path().is_ident("validate") {
            if let Ok(ts) = parse_validator_spec(attr) {
                return Some(ts);
            }

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
fn parse_validator_spec(attr: &Attribute) -> Result<proc_macro2::TokenStream, ()> {
    // Get the raw token stream inside the attribute parentheses
    let ts = attr.meta.require_list().map_err(|_| ())?.tokens.clone();
    let s = ts.to_string();

    // Extract the name before the first '('
    let open = s.find('(').ok_or(())?;
    let close = s.rfind(')').ok_or(())?;
    if close <= open { return Err(()); }
    let name = s[..open].trim();
    let inner = s[open + 1..close].trim();

    match name {
        "MaxLength" => {
            let limit: u32 = inner.parse().map_err(|_| ())?;
            let expr = quote! { ::validator::validators::max_length::MaxLength::new(#limit) };
            Ok(expr)
        }
        "MinLength" => {
            let limit: u32 = inner.parse().map_err(|_| ())?;
            let expr = quote! { ::validator::validators::min_length::MinLength::new(#limit) };
            Ok(expr)
        }
        "NotAllowedChars" => {
            let inner_tokens: proc_macro2::TokenStream = inner.parse().map_err(|_| ())?;
            let expr = quote! { ::validator::validators::not_allowed_chars::NotAllowedChars::new(#inner_tokens) };
            Ok(expr)
        }
        _ => Err(()),
    }
}