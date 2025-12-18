use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn sparkle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub beauty_index: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn enhance_beauty(&mut self, points: u64) {
                self.beauty_index += points;
                self.energy += points * 3; // Beauty attracts attention, generating energy
                println!("✨ Beauty enhanced by {} points. Beauty Index: {}. Energy: {}", points, self.beauty_index, self.energy);
            }

            pub fn dull_beauty(&mut self, points: u64) {
                self.beauty_index = self.beauty_index.saturating_sub(points);
                self.energy = self.energy.saturating_sub(points * 2); // Loss of beauty reduces energy
                println!("📉 Beauty dulled by {} points. Beauty Index: {}. Energy: {}", points, self.beauty_index, self.energy);
            }
        }
    };

    gen.into()
}
