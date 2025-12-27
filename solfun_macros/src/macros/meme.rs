extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "meme_impl", vis = "pub", hash = "d433e7fb")]
pub fn meme_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🐸 MEME! Generating/displaying meme about: \"{}\"\n", #description);
        ()
    }
    .into()
}