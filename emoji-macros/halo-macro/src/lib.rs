use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn halo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub virtue_score: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn cultivate_virtue(&mut self, points: u64) {
                self.virtue_score += points;
                self.energy += points / 2; // Virtue contributes to energy
                println!("😇 Cultivated {} virtue points. Total virtue: {}. Energy: {}", points, self.virtue_score, self.energy);
            }

            pub fn test_virtue(&mut self) -> bool {
                if self.virtue_score > 50 {
                    self.energy += 5; // Passing a virtue test increases energy
                    println!("✨ Virtue test passed! Virtue: {}. Energy: {}", self.virtue_score, self.energy);
                    true
                } else {
                    self.energy = self.energy.saturating_sub(5); // Failing a virtue test reduces energy
                    println!("📉 Virtue test failed. Virtue: {}. Energy: {}", self.virtue_score, self.energy);
                    false
                }
            }
        }
    };

    gen.into()
}
