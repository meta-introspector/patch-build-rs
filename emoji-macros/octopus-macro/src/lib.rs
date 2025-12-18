use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn octopus(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub github_activity: Vec<String> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn fetch_repo_info(&mut self, repo: &str) {
                self.github_activity.push(format!("Fetched info for {}", repo));
                self.energy += 10; // GitHub activity generates energy
                println!("🐙 Fetched info for {}. Activity log: {:?}. Energy: {}", repo, self.github_activity, self.energy);
            }

            pub fn contribute_to_repo(&mut self, repo: &str, contribution: &str) {
                self.github_activity.push(format!("Contributed '{}' to {}", contribution, repo));
                self.energy += 25; // Contributions generate more energy
                println!("📝 Contributed '{}' to {}. Activity log: {:?}. Energy: {}", contribution, repo, self.github_activity, self.energy);
            }
        }
    };

    gen.into()
}
