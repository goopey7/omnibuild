use quote::{quote, ToTokens, TokenStreamExt};
use std::{fs, path::Path};
use syn::{parse_file, ItemFn};

extern crate proc_macro;

#[proc_macro]
pub fn add_lua_functions(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = Path::new("src/lua/api.rs");

    let mut signatures = vec![];
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(parsed) = parse_file(&contents) {
            for item in parsed.items {
                if let syn::Item::Fn(ItemFn { sig, .. }) = item {
                    signatures.push(sig);
                }
            }
        }
    }

    let mut token_output = quote![];
    for sig in signatures {
        let ident = sig.ident;
        let mut inputs: Vec<syn::FnArg> = sig.inputs.into_iter().collect();

        let mut input_names: Vec<_> = inputs
            .iter()
            .filter_map(|input| {
                if let syn::FnArg::Typed(pat_type) = input {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some(pat_ident.ident.to_token_stream())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if inputs.is_empty() {
            token_output.append_all(quote![
                let func = lua.create_function(|_, ()| { Ok( super::api::#ident() )})?;
                ob_table.set(stringify!(#ident), func)?;
            ]);
        } else {
            token_output.append_all(quote![
                let func = lua.create_function(|#(#inputs),*| { Ok( super::api::#ident (#(#input_names),*) )})?;
                ob_table.set(stringify!(#ident), func)?;
            ]);
        }
    }
    token_output.into()
}
