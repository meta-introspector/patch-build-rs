use patch_build_rs_macros::mkbuildrs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

mkbuildrs! {
    module_name: "value";
    dependencies: ["proc_macro::TokenStream", "quote::quote", "syn"];
    description: "Set bounty values for fixme resolution";
    exports: ["value"];
}

pub fn value_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let bounty_amount = input_str.value();
    
    quote! {
        {
            println!("💰 Bounty set: {} for this fixme", #bounty_amount);
            #bounty_amount
        }
    }.into()
}
