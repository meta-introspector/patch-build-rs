use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn arrows(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub rebase_events: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn perform_rebase(&mut self) {
                self.rebase_events += 1;
                self.energy += 15; // Rebasing integrates changes, increasing energy
                println!("🔃 Rebase performed. Total rebase events: {}. Energy: {}", self.rebase_events, self.energy);
            }
        }
    };

    gen.into()
}
