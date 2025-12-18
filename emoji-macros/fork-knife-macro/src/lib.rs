use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn fork_knife(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub fork_count: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn create_fork(&mut self) {
                self.fork_count += 1;
                self.energy += 12; // Creating a fork expands possibilities, increasing energy
                println!("🍴 Fork created. Total forks: {}. Energy: {}", self.fork_count, self.energy);
            }

            pub fn merge_fork(&mut self) {
                if self.fork_count > 0 {
                    self.fork_count -= 1;
                    self.energy += 8; // Merging forks brings stability and energy
                    println!("🤝 Fork merged. Total forks: {}. Energy: {}", self.fork_count, self.energy);
                } else {
                    println!("⚠️ No forks to merge!");
                }
            }
        }
    };

    gen.into()
}
