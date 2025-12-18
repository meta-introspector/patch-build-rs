use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn gem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub resources: std::collections::HashMap<String, u64> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn add_resource(&mut self, name: String, amount: u64) {
                *self.resources.entry(name.clone()).or_insert(0) += amount;
                self.energy += amount / 5; // Resources contribute to energy
                println!("💎 Added {} {} resources. Total {}: {}. Energy: {}", amount, name, name, self.resources[&name], self.energy);
            }

            pub fn deplete_resource(&mut self, name: String, amount: u64) {
                if let Some(res_amount) = self.resources.get_mut(&name) {
                    if *res_amount >= amount {
                        *res_amount -= amount;
                        self.energy = self.energy.saturating_sub(amount / 10); // Depleting resources costs some energy
                        println!("⛏️ Depleted {} {} resources. Remaining {}: {}. Energy: {}", amount, name, name, *res_amount, self.energy);
                    } else {
                        println!("⚠️ Not enough {} resources to deplete {}!", name, amount);
                    }
                } else {
                    println!("⚠️ Resource {} not found!", name);
                }
            }
        }
    };

    gen.into()
}
