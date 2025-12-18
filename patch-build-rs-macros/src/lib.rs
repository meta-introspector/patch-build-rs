extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};

// Assuming Expr and PureProgram are accessible from the parent crate `patch_build_rs`
// This requires `patch-build-rs-macros/Cargo.toml` to have `patch-build-rs` as a dependency
// e.g., `patch-build-rs = { path = "../", optional = true }` and then enabled in features if needed,
// or directly if it's a library within the same workspace as `patch-build-rs-macros`.
// For simplicity, let's assume direct access if it's in the same workspace and properly linked.
// We'll use `patch_build_rs::expr::Expr` and `patch_build_rs::pure_program::PureProgram`.
#[allow(unused_imports)]
use introspector_core::{Expr, PureProgram};

#[proc_macro]
pub fn pure_reflect(input: TokenStream) -> TokenStream {
    let input_proc_macro2: proc_macro2::TokenStream = input.clone().into();
    let input_str = input_proc_macro2.to_string();

    let mut hasher = DefaultHasher::new();
    input_str.hash(&mut hasher);
    let hashed_value = hasher.finish();

    let mut set = BTreeSet::new();
    set.insert(hashed_value);

    let program_name = format!("reflected_program_{}", hashed_value);
    let _pure_program = PureProgram { set, name: program_name.clone() };

    // This token stream will construct the `(Expr, String)` tuple at compile time
    quote! {
        // Construct a BTreeSet<u64>
        {
            let mut set = std::collections::BTreeSet::new();
            set.insert(#hashed_value);
            let pure_program = introspector_core::PureProgram {
                set,
                name: #program_name.to_string(),
            };
            (introspector_core::Expr::PureAttractor(pure_program), #input_str.to_string())
        }
    }
    .into()
}

// Placeholder for grast!
#[proc_macro]
pub fn grast(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    quote! {
        format!("Conceptual RDF Turtle of: {}", #input_str)
    }
    .into()
}

// Placeholder for lru attribute macro
#[proc_macro_attribute]
pub fn lru(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let args_str = args.to_string();
    let fn_name = item_fn.sig.ident.to_string();

    eprintln!("\n✅ LRU! Conceptually applying LRU caching to function '{}' with args: '{}'\n", fn_name, args_str);

    quote! { #item_fn }.into()
}

// Placeholder for mkbuildrs!
#[proc_macro]
pub fn mkbuildrs(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    quote! {
        eprintln!("\n🏗️ MKBUILDRS! Conceptually generating build.rs content from: {}", #input_str);
        ()
    }
    .into()
}

#[proc_macro]
pub fn newquote(input: TokenStream) -> TokenStream {
    pure_reflect(input)
}



