use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn notes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub harmonic_balance: u664 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn achieve_harmony(&mut self, balance_points: u64) {
                self.harmonic_balance += balance_points;
                self.energy += balance_points * 6; // Achieving harmony creates significant energy
                println!("🎶 Harmony achieved! Balance: {}. Energy: {}", self.harmonic_balance, self.energy);
            }
        }
    };

    gen.into()
}
