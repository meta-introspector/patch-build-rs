use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn broom(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub filtered_items: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn filter_data(&mut self, items_to_filter: u64) {
                self.filtered_items += items_to_filter;
                self.energy += items_to_filter / 2; // Filtering data improves signal, increasing energy
                println!("🧹 Filtered {} items. Total filtered: {}. Energy: {}", items_to_filter, self.filtered_items, self.energy);
            }
        }
    };

    gen.into()
}
