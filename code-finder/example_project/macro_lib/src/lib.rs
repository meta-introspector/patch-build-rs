extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Ident, Token, parse::{Parse, ParseStream}, Result};

struct ProcessMatchInput {
    _file_token: Ident,
    _colon1: Token![:],
    file: LitStr,
    _comma1: Token![,],
    _line_token: Ident,
    _colon2: Token![:],
    line: syn::LitInt,
    _comma2: Token![,],
    _column_token: Ident,
    _colon3: Token![:],
    column: syn::LitInt,
    _comma3: Token![,],
    _text_token: Ident,
    _colon4: Token![:],
    text: LitStr,
}

impl Parse for ProcessMatchInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let _file_token = input.parse::<Ident>()?; // "file"
        let _colon1 = input.parse::<Token![:]>()?;
        let file = input.parse::<LitStr>()?;
        let _comma1 = input.parse::<Token![,]>()?;

        let _line_token = input.parse::<Ident>()?; // "line"
        let _colon2 = input.parse::<Token![:]>()?;
        let line = input.parse::<syn::LitInt>()?;
        let _comma2 = input.parse::<Token![,]>()?;

        let _column_token = input.parse::<Ident>()?; // "column"
        let _colon3 = input.parse::<Token![:]>()?;
        let column = input.parse::<syn::LitInt>()?;
        let _comma3 = input.parse::<Token![,]>()?;

        let _text_token = input.parse::<Ident>()?; // "text"
        let _colon4 = input.parse::<Token![:]>()?;
        let text = input.parse::<LitStr>()?;

        Ok(ProcessMatchInput {
            _file_token, _colon1, file, _comma1,
            _line_token, _colon2, line, _comma2,
            _column_token, _colon3, column, _comma3,
            _text_token, _colon4, text,
        })
    }
}

#[proc_macro]
pub fn process_match(input: TokenStream) -> TokenStream {
    let ProcessMatchInput {
        file,
        line,
        column,
        text,
        ..
    } = parse_macro_input!(input as ProcessMatchInput);

    // For demonstration, let's just print the information during compilation.
    // In a real scenario, you might generate code, write to a file, etc.
    let output = quote! {
        // You could generate struct definitions, push to a static array, etc.
        // For now, let's just log it to stdout during build for visibility.
        #[allow(unused_macros)]
        macro_rules! log_match_info {
            () => {
                // This would appear in build output
                // eprintln!(
                //     "Matched code in {}:{}:{}: {}",
                //     #file,
                //     #line,
                //     #column,
                //     #text
                // );
            };
        }
        log_match_info!();

        // Example: generate a placeholder function or constant that uses the info
        // This is where you would transform the 'grep result' into actual Rust code
        // For example, constructing a static array of all found matches:
        //
        // pub struct CodeMatch {
        //     pub file: &'static str,
        //     pub line: u32,
        //     pub column: u32,
        //     pub text: &'static str,
        // }
        //
        // inventory::submit! {
        //     CodeMatch {
        //         file: #file,
        //         line: #line,
        //         column: #column,
        //         text: #text,
        //     }
        // }
    };

    output.into()
}

// Example usage in build.rs:
//
// extern crate macro_lib; // Or path to your local macro lib
//
// fn main() {
//    macro_lib::process_match! {
//        file: "src/main.rs",
//        line: 10,
//        column: 5,
//        text: "some matched code"
//    };
// }
