use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn lock(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub is_locked: bool })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn lock_state(&mut self) {
                self.is_locked = true;
                self.energy = self.energy.saturating_sub(15); // Locking state costs energy
                println!("🔒 State locked. Energy: {}. Certain operations may be restricted.", self.energy);
            }

            pub fn unlock_state(&mut self) {
                self.is_locked = false;
                self.energy += 10; // Unlocking state can release energy
                println!("🔓 State unlocked. Energy: {}. All operations are now available.", self.energy);
            }
        }
    };

    gen.into()
}
