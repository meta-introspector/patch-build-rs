extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr, Token, parse::{Parse, ParseStream}, Result as SynResult};

struct ToolCallArgs {
    tool_name: LitStr,
    _comma: Token![,],
    args: LitStr,
}

impl Parse for ToolCallArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let tool_name = input.parse()?;
        let _comma = input.parse()?;
        let args = input.parse()?;
        Ok(ToolCallArgs { tool_name, _comma, args })
    }
}

pub fn toolcall_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as ToolCallArgs);

    let tool_name = args.tool_name;
    let args_value = args.args;
    let span = tool_name.span();

    let simulated_tool_output = format!("Tool '{}' executed with args: {}", tool_name.value(), args_value.value());

    quote_spanned! {span =>
        eprintln!("\n🛠️ TOOLCALL! Executing tool: `{}` with args: `{}`. Awaiting results...\n", #tool_name, #args_value);
        solfun_macros::results! {{ #simulated_tool_output }}
        ()
    }
    .into()
}