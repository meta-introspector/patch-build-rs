use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn ballot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub votes: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn cast_vote(&mut self) {
                self.votes += 1;
                self.energy += 1; // Voting consumes a little energy, but contributes to consensus
                println!("🗳️ Vote cast. Total votes: {}. Energy: {}", self.votes, self.energy);
            }

            pub fn tally_votes(&mut self) {
                if self.votes > 0 {
                    self.energy += self.votes / 10; // Tallying votes can generate energy based on participation
                    println!("📊 Votes tallied. Total votes: {}. Energy: {}", self.votes, self.energy);
                } else {
                    println!("❌ No votes to tally.");
                }
                self.votes = 0; // Reset votes after tallying
            }
        }
    };

    gen.into()
}
