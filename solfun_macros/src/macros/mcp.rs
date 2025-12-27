extern crate proc_macro;
use patch_build_rs_macros::decl;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

#[decl(fn, name = "mcp_impl", vis = "pub", hash = "7851d865")]
pub fn mcp_impl(input: TokenStream) -> TokenStream {
    let context_description = parse_macro_input!(input as LitStr);
    let span = context_description.span();

    quote_spanned! {span =>
        eprintln!("\n🌐 MCP! (Model Context Provider) Reifying context: '{}' into OWL properties.\n", #context_description);
        // Return a conceptual string, as mcp is used in an assignment
        "conceptual_contextual_report_from_mcp"
    }
    .into()
}