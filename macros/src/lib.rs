use quote::{quote, ToTokens, TokenStreamExt};
use std::{fs, path::Path};
use syn::{parse_file, ItemFn, Type, TypeReference};

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
        let inputs: Vec<syn::FnArg> = sig.inputs.into_iter().collect();

        // Check if first parameter is &mlua::Lua
        let first_param_is_lua = inputs.first().map_or(false, |input| {
            if let syn::FnArg::Typed(pat_type) = input {
                is_lua_type(&pat_type.ty)
            } else {
                false
            }
        });

        if first_param_is_lua {
            // Skip the first parameter (lua context) when collecting names and types
            let lua_inputs = &inputs[1..];
            let input_names: Vec<_> = lua_inputs
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
            let input_types: Vec<_> = lua_inputs
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_type) = input {
                        Some(pat_type.ty.to_token_stream())
                    } else {
                        None
                    }
                })
                .collect();

            if lua_inputs.is_empty() {
                // Function only takes lua context
                token_output.append_all(quote![
                    let func = lua.create_function(|lua, ()| { Ok( super::api::#ident(lua) )})?;
                    ob_table.set(stringify!(#ident), func)?;
                ]);
            } else {
                // Function takes lua context plus other parameters
                token_output.append_all(quote![
                    let func = lua.create_function(|lua, (#(#input_names),*): (#(#input_types),*)| {
                        Ok(super::api::#ident(lua, #(#input_names),*))
                    })?;
                    ob_table.set(stringify!(#ident), func)?;
                ]);
            }
        } else {
            // Original logic for functions without lua context
            let input_names: Vec<_> = inputs
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
            let input_types: Vec<_> = inputs
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_type) = input {
                        Some(pat_type.ty.to_token_stream())
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
                    let func = lua.create_function(|_, (#(#input_names),*): (#(#input_types),*)| {
                        Ok(super::api::#ident(#(#input_names),*))
                    })?;
                    ob_table.set(stringify!(#ident), func)?;
                ]);
            }
        }
    }
    token_output.into()
}

// Helper function to check if a type is &mlua::Lua
fn is_lua_type(ty: &Type) -> bool {
    if let Type::Reference(TypeReference { elem, .. }) = ty {
        if let Type::Path(type_path) = elem.as_ref() {
            // Check if the path ends with "Lua" and contains "mlua"
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident == "Lua" {
                    // Check if any segment contains "mlua"
                    return type_path
                        .path
                        .segments
                        .iter()
                        .any(|seg| seg.ident == "mlua");
                }
            }
        }
    }
    false
}
