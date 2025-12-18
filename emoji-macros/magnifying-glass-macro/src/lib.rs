use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn magnifying_glass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub truth_revealed: bool })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn seek_truth(&mut self) {
                self.truth_revealed = true;
                self.energy += 25; // Seeking truth brings clarity and energy
                println!("🔎 Truth revealed! Energy: {}", self.energy);
            }

            pub fn obscure_truth(&mut self) {
                self.truth_revealed = false;
                self.energy = self.energy.saturating_sub(15); // Obscuring truth can be costly
                println!("🙈 Truth obscured. Energy: {}", self.energy);
            }
        }
    };

    gen.into()
}
