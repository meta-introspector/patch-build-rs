use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn dna(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub adaptation_score: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn adapt_system(&mut self, challenge_level: u664) {
                self.adaptation_score += challenge_level;
                self.energy += challenge_level * 5; // Adapting consumes effort but makes the system stronger
                println!("🧬 System adapted to challenge level {}. Adaptation score: {}. Energy: {}", challenge_level, self.adaptation_score, self.energy);
            }
        }
    };

    gen.into()
}
