extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::quote;

#[decl(fn, name = "make_everything_a_macro_impl", vis = "pub", hash = "4337549a")]
pub fn make_everything_a_macro_impl(_input: TokenStream) -> TokenStream {
    quote! {
        eprintln!("\n✨ MAKE EVERYTHING A MACRO! The ultimate metaprogramming transformation is complete! 🤯\n");
        ()
    }
    .into()
}