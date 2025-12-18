extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr};

pub fn llm_impl(input: TokenStream) -> TokenStream {
    let request = parse_macro_input!(input as LitStr);
    let request_content = request.value();
    let span = request.span();

    let tool_name = "llm_text_generation_service";
    let tool_args = format!("request: \"{{}}\"", request_content); // Corrected format string

    quote_spanned! {span =>
        eprintln!("\n🧠 LLM! Interpreted request: \"{{}}\". Resolving to toolcall!\n", #request);
        solfun_macros::toolcall! {{ #tool_name, #tool_args }}
        () // Purely for debugging, LLM will return a value
    }
    .into()
}