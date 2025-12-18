use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn bandage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub patches_applied: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn apply_patch(&mut self) {
                self.patches_applied += 1;
                self.energy += 10; // Applying patches improves stability and generates energy
                println!("🩹 Patch applied. Total patches: {}. Energy: {}", self.patches_applied, self.energy);
            }
        }
    };

    gen.into()
}
