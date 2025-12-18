use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn chart(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub reduced_value: u664 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn reduce_data(&mut self, items_reduced: u64) {
                self.reduced_value += items_reduced;
                self.energy += items_reduced * 4; // Reducing data leads to efficiency and energy
                println!("📉 Reduced {} items. Total reduced value: {}. Energy: {}", items_reduced, self.reduced_value, self.energy);
            }
        }
    };

    gen.into()
}
