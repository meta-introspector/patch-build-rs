use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn robot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub agents: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn deploy_agent(&mut self, count: u64) {
                self.agents += count;
                self.energy += count * 5; // Deploying agents increases energy
                println!("🤖 Deployed {} agents. Total agents: {}. Energy: {}", count, self.agents, self.energy);
            }

            pub fn retract_agent(&mut self, count: u64) {
                if self.agents >= count {
                    self.agents -= count;
                    self.energy = self.energy.saturating_sub(count * 2); // Retracting agents reduces energy
                    println!("🪖 Retracted {} agents. Remaining agents: {}. Energy: {}", count, self.agents, self.energy);
                } else {
                    println!("⚠️ Not enough agents to retract {}!", count);
                }
            }
        }
    };

    gen.into()
}
