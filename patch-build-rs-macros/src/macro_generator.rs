use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

pub fn generate_common_macros_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let _config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔧 Generating common subexpression macros");
            
            let common_macros_code = r#"
// Auto-generated common subexpression macros

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
            "#;
            
            common_macros_code.to_string()
        }
    }.into()
}

pub fn mkbuildrs_with_macros_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔧 mkbuildrs with common macros: {}", #config);
            
            let build_rs_with_macros = r#"
// Enhanced build.rs with common macro utilities
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
            
            build_rs_with_macros.to_string()
        }
    }.into()
}
