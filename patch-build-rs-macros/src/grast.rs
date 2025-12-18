use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, File};

use grast_core::GrastDb; // Import GrastDb from our new core library

/// A procedural macro that converts Rust code into RDF Turtle representation
/// using the `grast_core` library.
///
/// This macro expands to a `&'static str` literal containing the Turtle representation.
///
/// Usage:
/// ```ignore
/// let turtle_output: &str = grast! {
///     pub fn my_function(arg1: u32, arg2: String) -> Vec<String> {
///         println!("Hello, {}!", arg1);
///         let result = vec![arg2];
///         result
///     }
/// };
/// ```
#[proc_macro]
pub fn grast(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a syn::File
    let ast_file = parse_macro_input!(input as File);

    // Create a GrastDb and flatten the AST into it
    let mut db = GrastDb::new();
    db.flatten(&ast_file);

    // Convert the GrastDb to its Turtle string representation
    let turtle_string = db.to_turtle();

    // The macro should expand to a string literal containing the turtle_string
    // We escape the string to ensure it's a valid Rust string literal.
    let expanded = quote! {
        #turtle_string
    };

    expanded.into()
}