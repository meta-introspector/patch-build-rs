extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "cargo_manipulate_impl", vis = "pub", hash = "963a392e")]
pub fn cargo_manipulate_impl(input: TokenStream) -> TokenStream {
    let action = parse_macro_input!(input as LitStr);
    let span = action.span();

    quote_spanned! {span =>
        eprintln!("\n📦 CARGO MANIPULATE! Action: {}\n", #action);
        ()
    }
    .into()
}