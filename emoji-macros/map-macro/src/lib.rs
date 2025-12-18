use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn map(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub mapped_elements: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn map_transformation(&mut self, elements_transformed: u64) {
                self.mapped_elements += elements_transformed;
                self.energy += elements_transformed * 3; // Transforming elements adds value and energy
                println!("🗺️ Transformed {} elements. Total mapped: {}. Energy: {}", elements_transformed, self.mapped_elements, self.energy);
            }
        }
    };

    gen.into()
}
