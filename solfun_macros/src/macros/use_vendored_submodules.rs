extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

pub fn use_vendored_submodules_impl(_input: TokenStream) -> TokenStream {
    quote! {
        eprintln!("\nVendored submodules configuration conceptually applied! 📦🔒\n");
        ()
    }
    .into()
}