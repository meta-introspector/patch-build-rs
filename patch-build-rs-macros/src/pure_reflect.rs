use patch_build_rs_macros::mkbuildrs;
use proc_macro::TokenStream;
use quote::quote;

mkbuildrs! {
    module_name: "pure_reflect";
    dependencies: ["proc_macro::TokenStream", "quote::quote", "introspector_core"];
    description: "Total reflection of code into Expr meta-model";
    exports: ["pure_reflect", "newquote", "eval_macro"];
}

pub fn pure_reflect_impl(input: TokenStream) -> TokenStream {
    newquote_implementation_default(input)
}

pub fn newquote_implementation_default(input: TokenStream) -> TokenStream {
    let input_proc_macro2: proc_macro2::TokenStream = input.clone().into();
    let input_str = input_proc_macro2.to_string();

    quote! {
        {
            use introspector_core::{Expr, PureProgram};
            use std::collections::BTreeSet;

            let current_input_str = #input_str.to_string();
            let mut set = BTreeSet::new();
            set.insert(42u64); // Placeholder hash
            let program_name = "reflected_program".to_string();
            let generated_pure_program = PureProgram { set, name: program_name };
            let generated_expr = Expr::Placeholder;

            (generated_expr, current_input_str)
        }
    }.into()
}
