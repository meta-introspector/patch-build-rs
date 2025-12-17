use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;
use syn::parse_macro_input; // Only parse_macro_input is needed here

/// Represents the input to the find_nix_rustc! macro.
/// Can be empty, or a single string literal for filtering.
mod macro_args {
    use syn::{parse::{Parse, ParseStream, Result as SynResult}, LitStr};

    pub struct FindRustcArgs {
        pub filter: Option<LitStr>,
    }

    impl Parse for FindRustcArgs {
        fn parse(input: ParseStream) -> SynResult<Self> {
            if input.is_empty() {
                Ok(FindRustcArgs { filter: None })
            } else {
                let filter_lit: LitStr = input.parse()?;
                Ok(FindRustcArgs { filter: Some(filter_lit) })
            }
        }
    }
}

/// A procedural macro to find Rust compilers in the Nix store.
/// It can optionally take a string literal as a filter.
/// Returns a Vec<String> of paths.
///
/// Usage:
/// `let rustc_paths: Vec<String> = find_nix_rustc!();`
/// `let rustc_filtered_paths: Vec<String> = find_nix_rustc!("1.91");`
#[proc_macro]
pub fn find_nix_rustc(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as macro_args::FindRustcArgs);
    let filter_str = args.filter.map(|lit| lit.value());

    let nix_store_path = Path::new("/nix/store");
    let mut rustc_paths = Vec::new();

    if let Ok(entries) = fs::read_dir(nix_store_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Look for common rustc paths within Nix store derivations
                let rustc_candidate_path = path.join("bin").join("rustc");
                if rustc_candidate_path.exists() && rustc_candidate_path.is_file() {
                    if let Some(path_str) = rustc_candidate_path.to_str() {
                        // Apply filter if present
                        if let Some(ref filter) = filter_str {
                            if path_str.contains(filter) {
                                rustc_paths.push(path_str.to_string());
                            }
                        } else {
                            rustc_paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    // Generate a TokenStream that represents a Vec<String>
    // Convert &str to String using .to_string()
    let quoted_paths = rustc_paths.iter().map(|s| quote! { #s.to_string() });
    let output = quote! {
        vec![#(#quoted_paths),*]
    };

    output.into()
}