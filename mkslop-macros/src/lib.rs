use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::LitStr;
use quote::ToTokens;

use mkslop_core::fix_cfg_format_string; // Import the core logic

/// A procedural macro to apply common auto-fixes to AI-generated code,
/// especially format string issues.
///
/// This specific invocation aims to fix the `invalid format string` error
/// that occurs when `format!` is given a string literal containing unescaped
/// quotes and braces that confuse its own parsing.
#[proc_macro]
pub fn mkslop(input: TokenStream) -> TokenStream {
    // Parse the input as a single LitStr, as mkbuildrs! will pass it a LitStr
    let input_lit = parse_macro_input!(input as LitStr);

    // Call the core logic to fix the string
    let fixed_lit = fix_cfg_format_string(input_lit);
    
    // Return the TokenStream representation of the fixed LitStr
    fixed_lit.to_token_stream().into()
}
