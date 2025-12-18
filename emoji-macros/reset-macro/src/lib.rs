use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn reset(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn reset_cycle(&mut self) {
                self.energy = 50; // Reset to a baseline energy level
                println!("🔄 Cycle Reset. Energy baseline established at {}.", self.energy);
            }
        }
    };

    gen.into()
}
