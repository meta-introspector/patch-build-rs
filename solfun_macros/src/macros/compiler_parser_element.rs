extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "compiler_parser_element_impl", vis = "pub", hash = "0eef2414")]
pub fn compiler_parser_element_impl(input: TokenStream) -> TokenStream {
    let element_name = parse_macro_input!(input as LitStr);
    let span = element_name.span();

    quote_spanned! {span =>
        eprintln!("\n🔍 COMPILER PARSER ELEMENT! Interacting with: {}\n", #element_name);
        ()
    }
    .into()
}