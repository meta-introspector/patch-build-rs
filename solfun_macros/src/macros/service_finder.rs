extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn service_finder_impl(input: TokenStream) -> TokenStream {
    let need_description = parse_macro_input!(input as LitStr);
    let span = need_description.span();

    let found_provider = format!("Found provider for need: \'{}\'", need_description.value());

    quote_spanned! {span =>
        eprintln!("\n🔍 SERVICE FINDER! Searching for provider for: \'{}\''. Result: \'{}\'.\n", #need_description, #found_provider);
        ()
    }
    .into()
}

