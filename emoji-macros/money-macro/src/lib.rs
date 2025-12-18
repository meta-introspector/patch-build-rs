use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn money(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub assets: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn acquire_assets(&mut self, amount: u64) {
                self.assets += amount;
                self.energy += amount / 10; // Assets contribute to energy
                println!("💰 Acquired {} assets. Total assets: {}. Energy: {}", amount, self.assets, self.energy);
            }

            pub fn deploy_assets(&mut self, amount: u64) {
                if self.assets >= amount {
                    self.assets -= amount;
                    self.energy = self.energy.saturating_sub(amount / 20); // Deploying assets costs some energy
                    println!("💸 Deployed {} assets. Remaining assets: {}. Energy: {}", amount, self.assets, self.energy);
                } else {
                    println!("⚠️ Not enough assets to deploy {}!", amount);
                }
            }
        }
    };

    gen.into()
}
