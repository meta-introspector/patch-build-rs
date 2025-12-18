use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct, Ident, Token, Result as SynResult, Field};
use syn::parse::{Parse, ParseStream};

// Helper struct for parsing attributes to be passed to define_emoji_macro!
struct DefineEmojiMacroArgs {
    macro_name_ident: Ident,
    _comma1: Token![,],
    field_name: Ident,
    _colon1: Token![:],
    field_type: syn::Type,
    _comma2: Token![,],
    methods: proc_macro2::TokenStream, // The block of methods for the Impl
}

impl Parse for DefineEmojiMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let macro_name_ident: Ident = input.parse()?;
        let _comma1: Token![,] = input.parse()?;
        let field_name: Ident = input.parse()?;
        let _colon1: Token![:] = input.parse()?;
        let field_type: syn::Type = input.parse()?;
        let _comma2: Token![,] = input.parse()?;
        let methods: proc_macro2::TokenStream = input.parse()?; // Parse the rest as a token stream for methods

        Ok(DefineEmojiMacroArgs {
            macro_name_ident,
            _comma1,
            field_name,
            _colon1,
            field_type,
            _comma2,
            methods,
        })
    }
}


/// A declarative macro for defining emoji-based attribute procedural macros.
///
/// This macro abstracts away the boilerplate of creating `#[proc_macro_attribute]`
/// functions that add a field to a struct and implement methods for that struct.
///
/// Usage:
/// ```ignore
/// define_emoji_macro! {
///     fork_knife, // The name of the proc macro function
///     fork_count: u64, // The field to add: name: Type
///     { // The methods to implement
///         pub fn create_fork(&mut self) {
///             self.fork_count += 1;
///             println!("🍴 Fork created. Total forks: {}", self.fork_count);
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn define_emoji_macro(input: TokenStream) -> TokenStream {
    let DefineEmojiMacroArgs {
        macro_name_ident,
        field_name,
        field_type,
        methods,
        ..
    } = parse_macro_input!(input as DefineEmojiMacroArgs);

    let expanded = quote! {
        #[proc_macro_attribute]
        pub fn #macro_name_ident(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut item_struct = syn::parse_macro_input!(item as syn::ItemStruct);

            if let syn::Fields::Named(ref mut fields) = item_struct.fields {
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub #field_name: #field_type })
                        .unwrap(),
                );
            }

            let struct_name = &item_struct.ident;

            let gen = quote! {
                #item_struct

                impl #struct_name {
                    #methods
                }
            };

            gen.into()
        }
    };
    expanded.into()
}