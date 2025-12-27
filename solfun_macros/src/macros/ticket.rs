extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "ticket_impl", vis = "pub", hash = "610caa5a")]
pub fn ticket_impl(input: TokenStream) -> TokenStream {
    let description = parse_macro_input!(input as LitStr);
    let span = description.span();

    quote_spanned! {span =>
        eprintln!("\n🎫 TICKET! Created new ticket: \"{{}}\"\n", #description);
        ()
    }
    .into()
}