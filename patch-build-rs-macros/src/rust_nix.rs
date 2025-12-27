use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use introspector_decl2_macros::decl2;
#[decl2(fn, name = "nix_rust_src_impl", vis = "pub", hash = "ba79e3cf")]
pub fn nix_rust_src_impl(_input: TokenStream) -> TokenStream {
    quote! {
        {
            use std::process::Command;
            
            let output = Command::new("nix-store")
                .args(&["--query", "--requisites", "/nix/store/*rustc*"])
                .output()
                .expect("Failed to query nix store");
                
            let rust_store_path = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("rustc") && line.contains("src"))
                .unwrap_or("/nix/store/rustc-src-not-found");
                
            println!("cargo:warning=🦀 Found Rust source: {}", rust_store_path);
            rust_store_path.to_string()
        }
    }.into()
}

#[decl2(fn, name = "extract_decl_impl", vis = "pub", hash = "e544fc67")]
pub fn extract_decl_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let rust_file = input_str.value();
    
    quote! {
        {
            use std::fs;
            
            let content = fs::read_to_string(#rust_file)
                .unwrap_or_else(|_| "// File not found".to_string());
                
            // Extract function declarations only
            let decls = content
                .lines()
                .filter(|line| line.trim_start().starts_with("pub fn") || 
                              line.trim_start().starts_with("fn"))
                .collect::<Vec<_>>()
                .join("\n");
                
            println!("cargo:warning=📦 Extracted {} declarations from {}", 
                decls.lines().count(), #rust_file);
            decls
        }
    }.into()
}

#[decl2(fn, name = "patch_rust_impl", vis = "pub", hash = "28c77f76")]
pub fn patch_rust_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let patch_desc = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔧 Would apply patch: {}", #patch_desc);
            println!("cargo:warning=💡 Use nix-build to create patched Rust");
            "patch-ready"
        }
    }.into()
}
