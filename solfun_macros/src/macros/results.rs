extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn results_impl(input: TokenStream) -> TokenStream {
    let output_literal = parse_macro_input!(input as LitStr);
    let span = output_literal.span();

    let encoded_vector_representation = format!("Vector encoding of: '{}'", output_literal.value());

    quote_spanned! {span =>
        eprintln!("\n📊 RESULTS! Output received: '{}'. Encoded to vector: '{}'.\n", #output_literal, #encoded_vector_representation);
        ()
    }
    .into()
}