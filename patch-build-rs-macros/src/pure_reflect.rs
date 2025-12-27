use patch_build_rs_macros::mkbuildrs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, LitStr};
use introspector_core::{Expr, PureProgram, EXPR_CACHE};
use std::hash::{Hash, Hasher};

mkbuildrs! {
    module_name: "pure_reflect";
    dependencies: ["proc_macro::TokenStream", "quote::quote", "introspector_core", "syn"];
    description: "Total reflection of code into Expr meta-model";
    exports: ["pure_reflect", "rewriterustinrust", "commit_cache"];
}

#[proc_macro]
#[decl(fn, name = "pure_reflect", vis = "pub", hash = "2c06d7bb")]
pub fn pure_reflect(input: TokenStream) -> TokenStream {
    let input_proc_macro2: proc_macro2::TokenStream = input.clone().into();
    let input_str = input_proc_macro2.to_string();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input_str.hash(&mut hasher);
    let item_hash = hasher.finish();

    let expr_to_return: Expr;

    if let Some(cached_expr) = EXPR_CACHE.get(&item_hash) {
        expr_to_return = cached_expr.clone();
    } else {
        let item = parse_macro_input!(input as Item);

        let new_expr = match item {
            Item::Fn(item_fn) => Expr::from_fn(item_fn),
            Item::Struct(item_struct) => Expr::from_struct(item_struct),
            Item::Enum(item_enum) => Expr::from_enum(item_enum),
            Item::Trait(item_trait) => Expr::from_trait(item_trait),
            Item::Impl(item_impl) => Expr::from_impl(item_impl),
            Item::Use(item_use) => Expr::from_use(item_use),
            Item::Mod(item_mod) => Expr::from_module(item_mod),
            Item::Static(item_static) => Expr::from_static(item_static),
            Item::Const(item_const) => Expr::from_const_item(item_const),
            _ => {
                return quote! { compile_error!("pure_reflect macro only supports functions, structs, enums, traits, impls, uses, modules, statics, and consts as input.") }
                    .into();
            }
        };
        
        expr_to_return = new_expr;
    }

    // Call hash_and_register_recursive for the main expression
    // This handles cache insertion, count update, and lattice building.
    expr_to_return.hash_and_register_recursive(None);

    // Expand to both the original input tokens and the Expr representation
    quote! {
        #input
        // The Expr representation might not always be directly useful in the expanded code itself,
        // but it has been registered in the global caches.
        // We could, for example, emit a static reference to the Expr data here if needed for runtime.
    }.into()
}

#[proc_macro]
#[decl(fn, name = "rewriterustinrust", vis = "pub", hash = "e1a8edb0")]
pub fn rewriterustinrust(input: TokenStream) -> TokenStream {
    // 1. Quoting Phase (Reflective Ascent) - Use pure_reflect internally to get the Expr
    let item = parse_macro_input!(input as Item);
    let mut expr = match item {
        Item::Fn(item_fn) => Expr::from_fn(item_fn),
        Item::Struct(item_struct) => Expr::from_struct(item_struct),
        Item::Enum(item_enum) => Expr::from_enum(item_enum),
        Item::Trait(item_trait) => Expr::from_trait(item_trait),
        Item::Impl(item_impl) => Expr::from_impl(item_impl),
        Item::Use(item_use) => Expr::from_use(item_use),
        Item::Mod(item_mod) => Expr::from_module(item_mod),
        Item::Static(item_static) => Expr::from_static(item_static),
        Item::Const(item_const) => Expr::from_const_item(item_const),
        _ => {
            return quote! { compile_error!("rewriterustinrust macro only supports functions, structs, enums, traits, impls, uses, modules, statics, and consts as input.") }
                .into();
        }
    };

    // 2. Modification Phase (Formal Reasoning) - Placeholder for now
    // For demonstration, let's say we want to modify a function's body
    // This is where mkslop! or direct Expr manipulation would come in.
    // As a simple placeholder, let's try to change the name of a function if it's a function.
    if let Expr::Function(ref mut ts) = expr {
        // This is a very simplistic modification: we're still operating on TokenStream inside Expr.
        // A real modification would parse `ts` further, modify its structure, and re-quote.
        // For now, this is just to show the modification 'hook'.
        // We'll leave `ts` as is, effectively making this a no-op modification for now.
        // The actual modification will come when integrating mkslop!
    }


    // 3. Execution Phase (Reflective Descent) - Lower the modified Expr back to TokenStream
    quote! { #expr }.into()
}

#[proc_macro]
#[decl(fn, name = "commit_cache", vis = "pub", hash = "9d62a316")]
pub fn commit_cache(input: TokenStream) -> TokenStream {
    let args = syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated
        .parse(input)
        .unwrap_or_else(|e| abort_call_site!(e));

    let mut args_iter = args.iter();

    let cache_file_path = args_iter.next().map(|lit| lit.value()).unwrap_or_else(|| {
        "/tmp/expr_cache.json".to_string()
    });

    let counts_file_path = args_iter.next().map(|lit| lit.value()).unwrap_or_else(|| {
        "/tmp/expr_counts.json".to_string()
    });

    let lattice_file_path = args_iter.next().map(|lit| lit.value()).unwrap_or_else(|| {
        "/tmp/expr_lattice.json".to_string()
    });

    // Expand to code that calls the write_cache_to_json and write_counts_to_json functions
    quote! {
        {
            use std::path::Path;
            use introspector_core::{write_cache_to_json, write_counts_to_json, write_lattice_to_json};

            let cache_path = Path::new(#cache_file_path);
            if let Err(e) = write_cache_to_json(cache_path) {
                // Emit a warning if saving fails
                eprintln!("Warning: Failed to save EXPR_CACHE to {}: {}", #cache_file_path, e);
            }

            let counts_path = Path::new(#counts_file_path);
            if let Err(e) = write_counts_to_json(counts_path) {
                // Emit a warning if saving fails
                eprintln!("Warning: Failed to save SUBEXPR_COUNTS to {}: {}", #counts_file_path, e);
            }

            let lattice_path = Path::new(#lattice_file_path);
            if let Err(e) = write_lattice_to_json(lattice_path) {
                // Emit a warning if saving fails
                eprintln!("Warning: Failed to save SUBEXPR_LATTICE to {}: {}", #lattice_file_path, e);
            }
        }
    }.into()
}