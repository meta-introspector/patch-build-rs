extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "cwm_impl", vis = "pub", hash = "be6bed90")]
pub fn cwm_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🧠 CWM! Conceptual interaction: \"{{}}\"\n", #description);
        ()
    }
    .into()
}