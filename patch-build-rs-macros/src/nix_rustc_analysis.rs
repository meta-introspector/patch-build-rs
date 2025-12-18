use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use walkdir::WalkDir;
use std::path::PathBuf;

/// A function-like procedural macro that attempts to analyze Rustc source code.
/// It takes a string literal representing the path to the Rustc source directory.
///
/// Usage:
/// trynixrustc!("/path/to/rustc/source");
#[proc_macro]
pub fn trynixrustc(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as LitStr);
    let rustc_source_path_str = path_lit.value();
    let rustc_source_path = PathBuf::from(rustc_source_path_str);

    if !rustc_source_path.exists() {
        return syn::Error::new_spanned(
            path_lit.span(),
            format!("Rustc source path does not exist: {}", rustc_source_path_str),
        )
        .to_compile_error()
        .into();
    }
    if !rustc_source_path.is_dir() {
        return syn::Error::new_spanned(
            path_lit.span(),
            format!("Rustc source path is not a directory: {}", rustc_source_path_str),
        )
        .to_compile_error()
        .into();
    }

    let mut found_crate_paths = Vec::new();

    for entry in WalkDir::new(&rustc_source_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if path.ends_with("src/lib.rs") || path.ends_with("src/main.rs") {
                // We found a potential crate root (or binary root)
                // The parent's parent should be the crate directory
                if let Some(crate_dir) = path.parent().and_then(|p| p.parent()) {
                    found_crate_paths.push(crate_dir.to_path_buf());
                }
            }
        }
    }

    let mut output = quote! {};
    if found_crate_paths.is_empty() {
        output = quote! {
            compile_error! { "No src/lib.rs or src/main.rs found in the provided path." }
        };
    } else {
        let paths_str = found_crate_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n- ");
        let msg = format!("Found the following crate directories:\n- {}", paths_str);
        output = quote! {
            // Use eprintln! to print during macro expansion, not compile_error!
            // or use a custom 'info!' macro if available in the project.
            eprintln!(#msg);
        };
    }

    output.into()
}
