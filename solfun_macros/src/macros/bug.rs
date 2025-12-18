extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

pub fn bug_impl(input: TokenStream) -> TokenStream {
    let bug_description = parse_macro_input!(input as LitStr);
    let span = bug_description.span();

    quote_spanned! {span =>
        eprintln!("\n🐛 BUG! Reported bug: \"{}\"\n", #bug_description);
        ()
    }
    .into()
}