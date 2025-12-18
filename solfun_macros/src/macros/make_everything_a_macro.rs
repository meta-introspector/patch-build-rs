extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

pub fn make_everything_a_macro_impl(_input: TokenStream) -> TokenStream {
    quote! {
        eprintln!("\n✨ MAKE EVERYTHING A MACRO! The ultimate metaprogramming transformation is complete! 🤯\n");
        ()
    }
    .into()
}