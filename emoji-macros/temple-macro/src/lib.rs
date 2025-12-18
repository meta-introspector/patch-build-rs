use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn temple(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub dao_members: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn form_dao(&mut self, members: u64) {
                self.dao_members += members;
                self.energy += members * 5; // Forming DAO increases energy
                println!("🏛️ DAO formed with {} new members. Total members: {}. Energy: {}", members, self.dao_members, self.energy);
            }

            pub fn dissolve_dao(&mut self, members: u64) {
                if self.dao_members >= members {
                    self.dao_members -= members;
                    self.energy = self.energy.saturating_sub(members * 2); // Dissolving DAO reduces energy
                    println!("💥 DAO dissolved {} members. Remaining members: {}. Energy: {}", members, self.dao_members, self.energy);
                } else {
                    println!("⚠️ Not enough DAO members to dissolve {}!", members);
                }
            }
        }
    };

    gen.into()
}
