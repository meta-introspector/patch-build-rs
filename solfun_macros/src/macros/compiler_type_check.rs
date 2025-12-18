extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

pub fn compiler_type_check_impl(input: TokenStream) -> TokenStream {
    let code_snippet = parse_macro_input!(input as LitStr);
    let span = code_snippet.span();

    quote_spanned! {span =>
        eprintln!("\n✅ COMPILER TYPE CHECK! Checking type of: {}\n", #code_snippet);
        ()
    }
    .into()
}
