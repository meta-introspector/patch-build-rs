extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "huggingface_impl", vis = "pub", hash = "5ed700cf")]
pub fn huggingface_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🤗 HUGGINGFACE! Conceptual interaction: \"{{}}\"\n", #description);
        ()
    }
    .into()
}