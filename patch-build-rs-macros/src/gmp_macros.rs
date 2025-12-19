// patch-build-rs-macros/src/gmp_macros.rs

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr, Ident, Token};
use syn::punctuated::Punctuated;

/// Macro for tracking Good Manufacturing Practice (GMP) metrics.
///
/// Usage: `gmp!("derivation_id", "test_passed")`
#[proc_macro]
pub fn gmp(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);

    if args.is_empty() {
        return quote_spanned! {
            proc_macro2::Span::call_site() =>
            compile_error!("gmp! macro requires at least one argument (e.g., \"derivation_id\").")
        }.into();
    }

    let derivation_id = args.first().unwrap().value();
    let metric = args.get(1).map(|l| l.value()).unwrap_or_else(|| "unspecified".to_string());

    quote! {
        {
            println!("cargo:warning=🔬 GMP Metric for Derivation {}: {}", #derivation_id, #metric);
            // Placeholder for actual GMP metric tracking logic
            true
        }
    }.into()
}
