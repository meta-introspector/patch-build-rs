use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn bullseye(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub current_goal: Option<String> })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let gen = quote! {
        #item_struct

        impl #struct_name {
            pub fn set_goal(&mut self, goal: &str) {
                self.current_goal = Some(goal.to_string());
                println!("🎯 Goal set: {}", goal);
                self.energy -= 10; // Setting a goal costs a little energy
            }

            pub fn is_goal_reasonable(&self) -> bool {
                if let Some(goal) = &self.current_goal {
                    // This is a simple heuristic for "reasonableness"
                    let reasonable = self.energy > 100 && self.complexity > 2 && self.assets > 50;
                    if reasonable {
                        println!("✅ Goal '{}' seems reasonable given current state. Energy: {}, Complexity: {}, Assets: {}", goal, self.energy, self.complexity, self.assets);
                    } else {
                        println!("❌ Goal '{}' might not be reasonable yet. Energy: {}, Complexity: {}, Assets: {}", goal, self.energy, self.complexity, self.assets);
                    }
                    reasonable
                } else {
                    println!("🤷 No goal set to evaluate reasonableness.");
                    false
                }
            }
        }
    };

    gen.into()
}
