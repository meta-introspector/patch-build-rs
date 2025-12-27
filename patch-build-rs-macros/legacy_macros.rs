// Legacy macros that need to be preserved

#[proc_macro]
#[decl(fn, name = "grast", vis = "pub", hash = "7679dd68")]
pub fn grast(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as File);
    let mut rdf_turtle_triplets = Vec::new();

    for item in file.items {
        if let syn::Item::Fn(func) = item {
            let func_name = func.sig.ident.to_string();
            rdf_turtle_triplets.push(format!(
                ":func_{} :type :FunctionDecl .",
                func_name
            ));
        }
    }

    let rdf_output = rdf_turtle_triplets.join("\n");
    quote! { #rdf_output }.into()
}

#[proc_macro_attribute]
#[decl(fn, name = "lru", vis = "pub", hash = "2877ec72")]
pub fn lru(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let args_str = args.to_string();
    let fn_name = item_fn.sig.ident.to_string();

    eprintln!("\n✅ LRU! Conceptually applying LRU caching to function '{}' with args: '{}'\n", fn_name, args_str);
    quote! { #item_fn }.into()
}

#[proc_macro]
#[decl(fn, name = "mkbuildrs", vis = "pub", hash = "56d3457d")]
pub fn mkbuildrs(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let escaped_input_str = input_str.replace("{", "{{").replace("}", "}}");

    quote! {
        eprintln!("\n🏗️ MKBUILDRS! Conceptually generating build.rs content from: {}", #escaped_input_str);
    }.into()
}