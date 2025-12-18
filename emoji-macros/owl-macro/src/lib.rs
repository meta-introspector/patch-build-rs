use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn owl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub metis_partitions: u664 })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn partition_graph(&mut self, num_partitions: u64) {
                self.metis_partitions = num_partitions;
                self.energy += num_partitions * 10; // Partitioning a graph is complex but efficient, increasing energy
                println!("🦉 Graph partitioned into {} sections. Metis partitions: {}. Energy: {}", num_partitions, self.metis_partitions, self.energy);
            }
        }
    };

    gen.into()
}
