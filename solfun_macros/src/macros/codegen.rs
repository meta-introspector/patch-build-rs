extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, LitStr};

pub fn codegen_impl(input: TokenStream) -> TokenStream {
    let code_literal = parse_macro_input!(input as LitStr);
    let span = code_literal.span();

    quote_spanned! {span =>
        eprintln!("\n🤖 CODOGEN! Generated code:\n```rust\n{}\n```\n", #code_literal);
        ()
    }
    .into()
}

