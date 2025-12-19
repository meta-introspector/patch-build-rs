use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Visibility, ReturnType};
use proc_macro2; // Import proc_macro2 for its TokenStream type
#[allow(unused_imports)]
use introspector_core::{Expr, PureProgram, NewQuoteTrait}; // Import NewQuoteTrait
// Removed `use std::hash::{DefaultHasher, Hash, Hasher}`;
// Removed `use std::str::FromStr`;
use syn::File; // Import File for parsing a whole Rust file
// Removed `use syn::parse::{Parse, ParseStream, Result as SynResult}`; // Removed custom parsing imports


#[proc_macro]
pub fn pure_reflect(input: TokenStream) -> TokenStream {
    __newquote_implementation_default(input)
}

// Placeholder for grast!
#[proc_macro]
pub fn grast(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as File);
    let mut rdf_turtle_triplets = Vec::new();

    // Iterate over the items in the file
    for item in file.items {
        if let syn::Item::Fn(func) = item {
            // Found a function declaration
            let func_name = func.sig.ident.to_string();
            let func_visibility = match func.vis {
                Visibility::Public(_) => "Public".to_string(),
                _ => "Inherited".to_string(), // Default to Inherited for other visibilities
            };

            // Generate RDF Turtle triplets for function declaration
            // Subject: Function URI (e.g., :func_calculate_sum)
            // Predicate: :type
            // Object: :FunctionDecl
            rdf_turtle_triplets.push(format!(
                ":func_{} :type :FunctionDecl .",
                func_name
            ));
            // Predicate: :name
            // Object: literal function name
            rdf_turtle_triplets.push(format!(
                ":func_{} :name \"{}\" .",
                func_name, func_name
            ));
            // Predicate: :visibility
            // Object: literal visibility
            rdf_turtle_triplets.push(format!(
                ":func_{} :visibility \"{}\" .",
                func_name, func_visibility
            ));

            // Basic parameters: just count for now
            let param_count = func.sig.inputs.len();
            rdf_turtle_triplets.push(format!(
                ":func_{} :paramCount \"{}\"^^xsd:integer .",
                func_name, param_count
            ));

            // Basic return type: just "exists" for now
            let has_return_type = matches!(func.sig.output, ReturnType::Type(_, _));
            if has_return_type {
                rdf_turtle_triplets.push(format!(
                    ":func_{} :hasReturnType \"true\"^^xsd:boolean .",
                    func_name
                ));
            } else {
                 rdf_turtle_triplets.push(format!(
                    ":func_{} :hasReturnType \"false\"^^xsd:boolean .",
                    func_name
                ));
            }
        }
        // TODO: Add logic for StructDecl, EnumDecl, TraitDecl, ModDecl, UseDecl
    }

    let rdf_output = rdf_turtle_triplets.join("\n");

    // Return the RDF Turtle string wrapped in a quote! block
    quote! {
        #rdf_output
    }
    .into()
}

#[proc_macro]
pub fn structural_grep(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();

    let rdf_data_expr_tokens_pm = iter.next().expect("Expected RDF data expression as first argument");
    let _comma_token_pm = iter.next().expect("Expected comma after RDF data expression"); // Consume comma
    let pattern_expr_tokens_pm = iter.next().expect("Expected pattern expression as second argument");

    // Correct conversion from proc_macro::TokenTree to proc_macro2::TokenStream
    let rdf_data_expr_tokens: proc_macro2::TokenStream = rdf_data_expr_tokens_pm.to_string().parse().unwrap();
    let pattern_expr_tokens: proc_macro2::TokenStream = pattern_expr_tokens_pm.to_string().parse().unwrap();


    // We can't evaluate these expressions at macro-expansion time.
    // Instead, we will generate code that evaluates them at runtime.

    quote! {
        {
            let rdf_turtle_content_runtime: String = #rdf_data_expr_tokens; // Evaluate the RDF data expression
            let pattern_runtime: String = #pattern_expr_tokens; // Evaluate the pattern expression

            let mut matched_lines = Vec::new();
            for line in rdf_turtle_content_runtime.lines() {
                if line.contains(&pattern_runtime) {
                    matched_lines.push(line.to_string());
                }
            }
            matched_lines
        }
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
    let escaped_input_str = input_str.replace("{", "{{").replace("}", "}}"); // Escape curly braces

    output_tokens.extend(quote! {
        eprintln!("\n🏗️ MKBUILDRS! Conceptually generating build.rs content from: {}", #escaped_input_str);
    });

    for (cfg_key, cfg_value) in parsed_input.cfgs {
        let cfg_key_str = cfg_key.value();
        let cfg_value_str = cfg_value.value();
        output_tokens.extend(quote! {
            println!("cargo:rustc-cfg={}=\"{{}}\"", #cfg_key_str, #cfg_value_str);
        });
    }

    for (check_cfg_key, check_cfg_values) in parsed_input.check_cfgs {
        let values_quoted = check_cfg_values.into_iter()
            .map(|s| format!("\"{}\"", s.value()))
            .collect::<Vec<String>>()
            .join(", ");
        let check_cfg_key_str = check_cfg_key.value(); // Extract value from LitStr
        output_tokens.extend(quote! {
            println!("cargo:rustc-check-cfg=cfg({} values({}))", #check_cfg_key_str, #values_quoted);
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
        let escaped_lib_rs_content = lib_rs_content.value().replace("{", "{{").replace("}", "}}");
        // Output a cargo:rustc-env instruction
        output_tokens.extend(quote! {
            println!("cargo:rustc-env=GENERATED_LIB_RS_CONTENT={}", #escaped_lib_rs_content);
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