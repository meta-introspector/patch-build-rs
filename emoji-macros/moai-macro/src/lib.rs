use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn moai(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub sculpt_value: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn sculpt_state(&mut self) {
                self.sculpt_value += 1;
                self.energy += 30; // Sculpting takes effort, but refines the system, increasing energy
                println!("🗿 State sculpted. Sculpt value: {}. Energy: {}", self.sculpt_value, self.energy);
            }
        }
    };

    gen.into()
}
