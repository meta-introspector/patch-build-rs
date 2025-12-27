extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "euler_impl", vis = "pub", hash = "b26f3bf9")]
pub fn euler_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🗺️ EULER! Conceptual interaction: \"{}\"\n", #description);
        ()
    }
    .into()
}