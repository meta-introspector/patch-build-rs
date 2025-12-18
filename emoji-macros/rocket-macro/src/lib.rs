use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn rocket(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub energy: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn launch(&mut self) {
                self.energy += 100;
                println!("🚀 Liftoff! Meme energy increased to {}!", self.energy);
            }

            pub fn decay(&mut self) {
                self.energy = self.energy.saturating_sub(10);
                println!("💨 Energy decayed to {}.", self.energy);
            }
        }
    };

    gen.into()
}
