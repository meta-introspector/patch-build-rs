extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn dwim_impl(input: TokenStream) -> TokenStream {
    let intent = parse_macro_input!(input as LitStr);
    let span = intent.span();

    quote_spanned! {span =>
        eprintln!("\n🧠 DWIM! Attempting to infer and execute intent: \"{}\"...\n", #intent);
        // Return a conceptual string, as dwim is used in an assignment
        "conceptual_raw_report_from_dwim"
    }
    .into()
}