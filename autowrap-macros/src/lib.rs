use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::process::Command;
use std::fs;

pub mod extract;
pub mod refactor;

pub use extract::*;
pub use refactor::*;

#[proc_macro]
pub fn autowrap(input: TokenStream) -> TokenStream {
    let code = parse_macro_input!(input as LitStr).value();
    
    // Try compiling raw code first
    if let Ok(wrapped) = try_compile_and_wrap(&code) {
        return wrapped.parse().unwrap();
    }
    
    // Fallback: basic wrapping
    let output = quote! {
        {
            prelude! {
                use std::*;
            }
            mkdecl! {
                #code
            }
        }
    };
    output.into()
}

fn try_compile_and_wrap(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let test_code = format!(r#"
fn main() {{
    {}
}}
"#, code);
    
    fs::write("/tmp/test_snippet.rs", &test_code)?;
    
    let output = Command::new("rustc")
        .args(&["--crate-type", "bin", "/tmp/test_snippet.rs", "-o", "/tmp/test_snippet"])
        .output()?;
    
    if output.status.success() {
        Ok(format!(r#"
prelude! {{
    // Auto-detected: no imports needed
}}
mkdecl! {{
    {}
}}
"#, code))
    } else {
        // Parse errors and add common imports
        let stderr = String::from_utf8_lossy(&output.stderr);
        let imports = if stderr.contains("HashMap") {
            "use std::collections::HashMap;"
        } else if stderr.contains("Vec") {
            "use std::vec::Vec;"
        } else {
            "use std::*;"
        };
        
        Ok(format!(r#"
prelude! {{
    {}
}}
mkdecl! {{
    {}
}}
"#, imports, code))
    }
}

#[proc_macro]
pub fn prelude(input: TokenStream) -> TokenStream {
    input // Pass through for now
}

#[proc_macro]
pub fn mkdecl(input: TokenStream) -> TokenStream {
    input // Pass through for now
}
