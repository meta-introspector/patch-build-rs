extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn oil_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🛢️ OIL! Conceptual interaction: \"{}\"\n", #description);
        ()
    }
    .into()
}