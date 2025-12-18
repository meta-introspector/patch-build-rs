extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn fork_all_repos_impl(input: TokenStream) -> TokenStream {
    let org_name = parse_macro_input!(input as LitStr);
    let span = org_name.span();

    quote_spanned! {span =>
        eprintln!("\n🍴 FORK ALL REPOS! Conceptually forking all repos in organization: {}\n", #org_name);
        ()
    }
    .into()
}
