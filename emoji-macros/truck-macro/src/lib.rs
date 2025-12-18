use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn truck(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub vendored_deps: Vec<String> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn vendor_dependency(&mut self, dep_name: &str) {
                self.vendored_deps.push(dep_name.to_string());
                self.energy += 7; // Vendoring dependencies stabilizes the system, increasing energy
                println!("🚚 Vendored dependency: {}. Total vendored: {}. Energy: {}", dep_name, self.vendored_deps.len(), self.energy);
            }

            pub fn remove_vendored_dependency(&mut self, dep_name: &str) {
                let initial_len = self.vendored_deps.len();
                self.vendored_deps.retain(|d| d != dep_name);
                if self.vendored_deps.len() < initial_len {
                    self.energy = self.energy.saturating_sub(5); // Removing vendored dependency can introduce instability
                    println!("🗑️ Removed vendored dependency: {}. Remaining: {}. Energy: {}", dep_name, self.vendored_deps.len(), self.energy);
                } else {
                    println!("⚠️ Vendored dependency {} not found to remove!", dep_name);
                }
            }
        }
    };

    gen.into()
}
