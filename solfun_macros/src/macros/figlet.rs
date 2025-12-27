extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "figlet_impl", vis = "pub", hash = "e0e55bdf")]
pub fn figlet_impl(input: TokenStream) -> TokenStream {
    let text_literal = parse_macro_input!(input as LitStr);
    let span = text_literal.span();

    let simulated_figlet_output = format!("FIGLET: {}", text_literal.value());

    quote_spanned! {span =>
        eprintln!("\n{}\n", #simulated_figlet_output);
        () // Return unit to allow the macro to be used as a statement
    }
    .into()
}