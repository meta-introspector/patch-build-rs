extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

pub fn branch_impl(input: TokenStream) -> TokenStream {
    let branch_name = parse_macro_input!(input as LitStr);
    let span = branch_name.span();

    quote_spanned! {span =>
        eprintln!("\n🌿 BRANCH! Created new branch: `{}`\n", #branch_name);
        ()
    }
    .into()
}
