use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::process::Command;

#[proc_macro]
pub fn extract_and_wrap(input: TokenStream) -> TokenStream {
    let pattern = parse_macro_input!(input as LitStr).value();
    
    let output = Command::new("rg")
        .args(&["-A", "10", &pattern, ".", "--type", "rust"])
        .output()
        .expect("Failed to run ripgrep");
    
    let matches = String::from_utf8_lossy(&output.stdout);
    let code_snippets: Vec<&str> = matches
        .lines()
        .filter(|line| !line.starts_with("--") && !line.contains(".rs:"))
        .collect();
    
    let wrapped_snippets = code_snippets.iter().map(|snippet| {
        quote! {
            autowrap!(#snippet);
        }
    });
    
    let result = quote! {
        {
            #(#wrapped_snippets)*
        }
    };
    
    result.into()
}
