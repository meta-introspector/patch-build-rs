use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, parse_str};

pub fn checktemplate_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let template_code = input_str.value();
    
    // Try to parse the template code as Rust syntax
    let parse_result = parse_str::<syn::File>(&template_code);
    
    match parse_result {
        Ok(_) => {
            quote! {
                {
                    println!("cargo:warning=✅ Template syntax check passed");
                    #template_code
                }
            }.into()
        }
        Err(e) => {
            let error_msg = format!("❌ Template syntax error: {}", e);
            quote! {
                {
                    println!("cargo:warning={}", #error_msg);
                    compile_error!(#error_msg);
                }
            }.into()
        }
    }
}

pub fn generate_checked_macros_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let _config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔧 Generating syntax-checked common macros");
            
            // Template code that will be syntax-checked
            let common_macros_template = r#"
// Auto-generated common subexpression macros - SYNTAX CHECKED

/// Macro for common quote pattern: quote! { ... }.into()
macro_rules! quote_into {
    ($($tokens:tt)*) => {
        quote! { $($tokens)* }.into()
    };
}

/// Macro for common warning pattern
macro_rules! build_warning {
    ($msg:expr) => {
        println!("cargo:warning={}", $msg)
    };
    ($fmt:expr, $($args:expr),*) => {
        println!("cargo:warning={}", format!($fmt, $($args),*))
    };
}

/// Macro for common input parsing
macro_rules! parse_string_input {
    ($input:expr) => {
        parse_macro_input!($input as LitStr).value()
    };
}

/// Macro for common format pattern
macro_rules! format_template {
    ($template:expr, $($args:expr),*) => {
        format!($template, $($args),*)
    };
}

/// Macro for common vector creation
macro_rules! items_vec {
    ($($item:expr),*) => {
        vec![$($item),*]
    };
}

/// Macro for common error handling
macro_rules! unwrap_or_default {
    ($expr:expr, $default:expr) => {
        $expr.unwrap_or_else(|_| $default)
    };
}

/// Macro for common file writing
macro_rules! write_file {
    ($path:expr, $content:expr) => {
        std::fs::write($path, $content).ok()
    };
}

/// Macro for common proc_macro function pattern
macro_rules! proc_macro_fn {
    ($name:ident, $impl_fn:path) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $impl_fn(input)
        }
    };
}
            "#;
            
            // Perform compile-time syntax check
            let checked_template = crate::checktemplate!(common_macros_template);
            checked_template
        }
    }.into()
}

pub fn mkbuildrs_checked_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let config = input_str.value();
    
    // Simple build.rs template - pre-validated
    let build_rs_template = r#"
use std::process::Command;

macro_rules! build_warning {
    ($msg:expr) => {
        println!("cargo:warning={}", $msg)
    };
}

macro_rules! unwrap_or_default {
    ($expr:expr, $default:expr) => {
        $expr.unwrap_or_else(|_| $default)
    };
}

fn main() {
    build_warning!("🔧 Enhanced build starting...");
    
    let nix_available = unwrap_or_default!(
        Command::new("which").arg("nix").output().map(|o| o.status.success()),
        false
    );
    
    if nix_available {
        build_warning!("✅ Nix available");
    } else {
        build_warning!("⚠️ Nix not available");
    }
    
    build_warning!("🔧 Enhanced build complete");
}
    "#;
    
    // Validate the template at compile time
    let parse_result = parse_str::<syn::File>(build_rs_template);
    
    match parse_result {
        Ok(_) => {
            quote! {
                {
                    println!("cargo:warning=🔧 mkbuildrs with syntax-checked macros: {}", #config);
                    println!("cargo:warning=✅ Build template syntax validated");
                    #build_rs_template.to_string()
                }
            }.into()
        }
        Err(e) => {
            let error_msg = format!("❌ Build template syntax error: {}", e);
            quote! {
                {
                    println!("cargo:warning={}", #error_msg);
                    compile_error!(#error_msg);
                }
            }.into()
        }
    }
}
