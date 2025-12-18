use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn web(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub graph_complexity: u64 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn analyze_graph(&mut self) {
                self.graph_complexity += 10; // Analyzing graph increases complexity
                self.energy += 20; // But also generates energy
                println!("🕸️ Graph analyzed. Complexity: {}. Energy: {}", self.graph_complexity, self.energy);
            }
        }
    };

    gen.into()
}
