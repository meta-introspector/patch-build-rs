extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn replace_all_git_hub_actions_impl(input: TokenStream) -> TokenStream {
    let new_workflow = parse_macro_input!(input as LitStr);
    let span = new_workflow.span();

    quote_spanned! {span =>
        eprintln!("\n♻️ REPLACE ALL GITHUB ACTIONS! Conceptually replacing with: {}\n", #new_workflow);
        ()
    }
    .into()
}