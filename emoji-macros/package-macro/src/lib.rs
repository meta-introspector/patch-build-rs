use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn package(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub crates: Vec<String> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn add_crate(&mut self, crate_name: &str) {
                self.crates.push(crate_name.to_string());
                self.energy += 5; // Adding a crate increases energy
                println!("📦 Added crate: {}. Total crates: {}. Energy: {}", crate_name, self.crates.len(), self.energy);
            }

            pub fn remove_crate(&mut self, crate_name: &str) {
                let initial_len = self.crates.len();
                self.crates.retain(|c| c != crate_name);
                if self.crates.len() < initial_len {
                    self.energy = self.energy.saturating_sub(5); // Removing a crate decreases energy
                    println!("🗑️ Removed crate: {}. Total crates: {}. Energy: {}", crate_name, self.crates.len(), self.energy);
                } else {
                    println!("⚠️ Crate {} not found to remove!", crate_name);
                }
            }
        }
    };

    gen.into()
}
