use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn forty_two(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub ultimate_answer: Option<u64> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn find_ultimate_answer(&mut self) {
                self.ultimate_answer = Some(42);
                self.energy += 420; // Finding the ultimate answer brings immense energy!
                println!("🔢 The ultimate answer is revealed: {}. Energy: {}", self.ultimate_answer.unwrap(), self.energy);
            }
        }
    };

    gen.into()
}
