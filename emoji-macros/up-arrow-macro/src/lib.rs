use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn up_arrow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub abstraction_level: u664 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn lift_abstraction(&mut self) {
                self.abstraction_level += 1;
                self.energy += 20; // Lifting abstraction increases understanding and energy
                println!("⬆️ Abstraction level lifted to {}. Energy: {}", self.abstraction_level, self.energy);
            }
        }
    };

    gen.into()
}
