extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "fork_all_repos_impl", vis = "pub", hash = "eaae29c6")]
pub fn fork_all_repos_impl(input: TokenStream) -> TokenStream {
    let org_name = parse_macro_input!(input as LitStr);
    let span = org_name.span();

    quote_spanned! {span =>
        eprintln!("\n🍴 FORK ALL REPOS! Conceptually forking all repos in organization: {}\n", #org_name);
        ()
    }
    .into()
}