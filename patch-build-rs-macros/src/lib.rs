use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use proc_macro2; // Import proc_macro2 for its TokenStream type
#[allow(unused_imports)]
use introspector_core::{Expr, PureProgram, NewQuoteTrait}; // Import NewQuoteTrait
// Removed `use std::hash::{DefaultHasher, Hash, Hasher};`
// Removed `use std::str::FromStr;`

#[proc_macro]
pub fn pure_reflect(input: TokenStream) -> TokenStream {
    __newquote_implementation_default(input)
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

#[proc_macro]
pub fn mkbuildrs(input: TokenStream) -> TokenStream {
    let mut output_tokens = proc_macro2::TokenStream::new();
    let parsed_input = syn::parse_macro_input!(input as MkBuildRsConfigInput);

    // Existing logic from old mkbuildrs:
    let input_str = parsed_input.original_input_str; // Store original input string

    output_tokens.extend(quote! {
        eprintln!("\n🏗️ MKBUILDRS! Conceptually generating build.rs content from: {}", #input_str);
    });

    for (cfg_key, cfg_value) in parsed_input.cfgs {
        let cfg_key_str = cfg_key.value();
        let cfg_value_str = cfg_value.value();
        output_tokens.extend(quote! {
            println!("cargo:rustc-cfg={}=\"{}\"", #cfg_key_str, #cfg_value_str);
        });
    }

    for (check_cfg_key, check_cfg_values) in parsed_input.check_cfgs {
        let values_quoted = check_cfg_values.into_iter()
            .map(|s| format!("\"{}\"", s.value()))
            .collect::<Vec<String>>()
            .join(", ");
        let check_cfg_key_str = check_cfg_key.value(); // Extract value from LitStr
        output_tokens.extend(quote! {
            println!("cargo:rustc-check-cfg=cfg({}, values({}))", #check_cfg_key_str, #values_quoted);
        });
    }

    if let Some(resource_req) = parsed_input.resource_req {
        let resource_req_str = resource_req.to_string(); // Convert TokenStream to String
        output_tokens.extend(quote! {
            println!("cargo:resource_req={}", #resource_req_str);
        });
    }

    for secret in parsed_input.secret_req {
        output_tokens.extend(quote! {
            println!("cargo:secret_req={}", #secret);
        });
    }


    if let Some(lib_rs_content) = parsed_input.generate_lib_rs {
        // Output a cargo:rustc-env instruction
        output_tokens.extend(quote! {
            println!("cargo:rustc-env=GENERATED_LIB_RS_CONTENT={}", #lib_rs_content);
        });
    }

    output_tokens.into()
}

// Define ConfigInput for mkbuildrs!
struct MkBuildRsConfigInput {
    generate_lib_rs: Option<syn::LitStr>,
    cfgs: Vec<(syn::LitStr, syn::LitStr)>, // Stores (key, value) for cfg:
    check_cfgs: Vec<(syn::LitStr, Vec<syn::LitStr>)>, // Stores (key, values) for check_cfg:
    resource_req: Option<proc_macro2::TokenStream>, // Stores the block directly
    secret_req: Vec<syn::LitStr>, // Stores list of secrets
    original_input_str: String,
}

impl syn::parse::Parse for MkBuildRsConfigInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let original_input_str = input.to_string(); // Capture original input string
        let mut generate_lib_rs = None;
        let mut cfgs = Vec::new();
        let mut check_cfgs = Vec::new();
        let mut resource_req = None;
        let mut secret_req = Vec::new();

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<syn::Token![:]>()?; // Parse ':'

            if key == "generate_lib_rs" {
                let content: syn::LitStr = input.parse()?;
                generate_lib_rs = Some(content);
            } else if key == "cfg" {
                let cfg_key: syn::LitStr = input.parse()?;
                input.parse::<syn::Token![=]>()?; // Parse '='
                let cfg_value: syn::LitStr = input.parse()?;
                cfgs.push((cfg_key, cfg_value));
            } else if key == "check_cfg" {
                let check_cfg_key: syn::LitStr = input.parse()?;
                input.parse::<syn::Token![,]>()?; // Parse ','
                let check_cfg_values_ident: syn::Ident = input.parse()?; // Expect 'values'
                if check_cfg_values_ident != "values" {
                    return Err(input.error("expected 'values' after check_cfg key"));
                }
                input.parse::<syn::Token![=]>()?; // Parse '='
                let values_content;
                syn::bracketed!(values_content in input); // Parse [...]
                let check_cfg_values: syn::punctuated::Punctuated<syn::LitStr, syn::Token![,]> =
                    values_content.parse_terminated(|parser_input: syn::parse::ParseStream| parser_input.parse::<syn::LitStr>(), syn::Token![,])?;
                check_cfgs.push((check_cfg_key, check_cfg_values.into_iter().collect())); // Fixed here
            } else if key == "resource_req" {
                let content;
                syn::braced!(content in input); // Parse { ... }
                resource_req = Some(content.parse()?); // Capture the whole block as TokenStream
            } else if key == "secret_req" {
                let content;
                syn::bracketed!(content in input); // Parse [...]
                let secrets: syn::punctuated::Punctuated<syn::LitStr, syn::Token![,]> =
                    content.parse_terminated(|parser_input: syn::parse::ParseStream| parser_input.parse::<syn::LitStr>(), syn::Token![,])?;
                secret_req.extend(secrets.into_iter());
            } else {
                return Err(input.error(format!("unexpected key: {}", key)));
            }

            // Parse optional semicolon
            if input.peek(syn::Token![;]) {
                input.parse::<syn::Token![;]>()?;
            }
        }
        Ok(MkBuildRsConfigInput {
            generate_lib_rs,
            cfgs,
            check_cfgs,
            resource_req,
            secret_req,
            original_input_str,
        })
    }
}

#[proc_macro]
#[doc(hidden)] // Hide from documentation as it's an internal detail
pub fn __quote_implementation(input: TokenStream) -> TokenStream {
    // By default, __quote_implementation simply delegates to the original quote! macro.
    // Users can override this by defining their own __quote_implementation in a higher-priority crate.
    let input_proc_macro2: proc_macro2::TokenStream = input.into();
    quote! { #input_proc_macro2 }.into()
}

#[proc_macro]
#[doc(hidden)] // Hide from documentation as it's an internal detail
pub fn __newquote_implementation(input: TokenStream) -> TokenStream {
    // By default, __newquote_implementation calls __newquote_implementation_default.
    // Users can override this by defining their own __newquote_implementation in a higher-priority crate.
    __newquote_implementation_default(input)
}

#[proc_macro]
pub fn __newquote_implementation_default(input: TokenStream) -> TokenStream {
    let input_proc_macro2: proc_macro2::TokenStream = input.clone().into();
    let input_str = input_proc_macro2.to_string();

    quote! {
        {
            use std::str::FromStr;
            use introspector_core::{Expr, PureProgram, NewQuoteTrait};
            use std::hash::{DefaultHasher, Hash, Hasher};

            let current_input_str = #input_str.to_string();

            let mut runtime_hasher = DefaultHasher::new();
            current_input_str.hash(&mut runtime_hasher);
            let runtime_hashed_value = runtime_hasher.finish();

            let mut set = std::collections::BTreeSet::new();
            set.insert(runtime_hashed_value);
            let program_name = format!("reflected_program_{}", runtime_hashed_value);
            let generated_pure_program = introspector_core::PureProgram { set, name: program_name };
            let generated_expr = introspector_core::Expr::PureAttractor(generated_pure_program);

            let string_to_return = current_input_str;

            // Acquire lock on the cache
            let mut cache = introspector_core::EXPR_CACHE.lock().unwrap();

            // Check if the Expr is already in the cache
            if let Some((cached_expr, cached_str)) = cache.get(&runtime_hashed_value) {
                (cached_expr.clone(), cached_str.clone())
            } else {
                eprintln!("📝 Quoted Object (Expr) for hash {}:\n{:#?}", runtime_hashed_value, generated_expr);
                cache.put(runtime_hashed_value, (generated_expr.clone(), string_to_return.clone()));
                (generated_expr, string_to_return)
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn newquote(input: TokenStream) -> TokenStream {
    // This is the user-facing macro that can be overridden.
    // By default, it calls __quote_implementation, which in turn calls the original quote!.
    __quote_implementation(input)
}

#[proc_macro]
pub fn eval_macro(input: TokenStream) -> TokenStream {
    // Phase 1: Capture the AST and store it using __newquote_implementation_default!
    // This will print the Expr and store it in the LRU cache.
    let _expr_capture_tokens = __newquote_implementation_default(input.clone()); // Clone input to use it again

    // Phase 2: Return the original input, after the side effect of __newquote_implementation_default! has happened.
    // This ensures that the AST is captured, but the original macro still expands.
    input
}



