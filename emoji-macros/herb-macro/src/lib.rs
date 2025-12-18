use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn herb(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub active_branches: Vec<String> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn create_branch(&mut self, branch_name: &str) {
                self.active_branches.push(branch_name.to_string());
                self.energy += 10; // New branches increase energy
                println!("🌿 Branch '{}' created. Active branches: {:?}. Energy: {}", branch_name, self.active_branches, self.energy);
            }

            pub fn switch_branch(&mut self, branch_name: &str) {
                if self.active_branches.contains(&branch_name.to_string()) {
                    self.energy += 2; // Switching branches is a low-cost operation
                    println!("🌱 Switched to branch '{}'. Energy: {}", branch_name, self.energy);
                } else {
                    println!("⚠️ Branch '{}' not found. Cannot switch.", branch_name);
                }
            }
        }
    };

    gen.into()
}
