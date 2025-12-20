use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, ItemFn, ItemStruct, ItemEnum, ItemTrait,
    ItemConst, ItemType, Attribute, Meta, MetaList, Lit, Ident,
};
use introspector_decl_common::{DeclInfo, register_decl};

pub fn decl_attr_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    
    // Parse the attribute arguments
    let args = parse_macro_input!(attr as DeclArgs);
    
    // Try to parse as different item types and extract metadata
    if let Ok(mut item_fn) = syn::parse::<ItemFn>(item_clone.clone()) {
        return wrap_function(&args, &mut item_fn);
    }
    
    if let Ok(mut item_struct) = syn::parse::<ItemStruct>(item_clone.clone()) {
        return wrap_struct(&args, &mut item_struct);
    }
    
    if let Ok(mut item_enum) = syn::parse::<ItemEnum>(item_clone.clone()) {
        return wrap_enum(&args, &mut item_enum);
    }
    
    if let Ok(mut item_trait) = syn::parse::<ItemTrait>(item_clone.clone()) {
        return wrap_trait(&args, &mut item_trait);
    }
    
    // If we can't parse, just return the original
    item
}

struct DeclArgs {
    node_type: Option<String>,
    name: Option<String>,
    vis: Option<String>,
    hash: Option<String>,
    extra: Vec<(String, String)>,
}

impl syn::parse::Parse for DeclArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = DeclArgs {
            node_type: None,
            name: None,
            vis: None,
            hash: None,
            extra: vec![],
        };
        
        // First token might be the node type
        if input.peek(syn::Ident) {
            let ident: Ident = input.parse()?;
            args.node_type = Some(ident.to_string());
            
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        
        // Parse key = value pairs
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let value: syn::LitStr = input.parse()?;
            
            match key.to_string().as_str() {
                "name" => args.name = Some(value.value()),
                "vis" => args.vis = Some(value.value()),
                "hash" => args.hash = Some(value.value()),
                other => args.extra.push((other.to_string(), value.value())),
            }
            
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        
        Ok(args)
    }
}

fn compute_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:08x}", hasher.finish() & 0xFFFFFFFF)
}

fn wrap_function(args: &DeclArgs, item: &mut ItemFn) -> TokenStream {
    let name = item.sig.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));
    
    let vis_str = match &item.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Restricted(_) => "pub(restricted)",
        syn::Visibility::Inherited => "private",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();
    
    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            use introspector_decl_common::{DeclInfo, register_decl};
            register_decl(DeclInfo {
                node_type: "fn",
                name: #name,
                visibility: #vis_str,
                module: #module_path_str,
                file: file!(), // Use file!() from the context where the macro is expanded
                line: #line,
                hash: #hash,
            });
        };
    };
    
    let output = quote! {
        #item
        #registration_code
    };
    
    output.into()
}

fn wrap_struct(args: &DeclArgs, item: &mut ItemStruct) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));
    
    let vis_str = match &item.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Restricted(_) => "pub(restricted)",
        syn::Visibility::Inherited => "private",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();
    
    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            use introspector_decl_common::{DeclInfo, register_decl};
            register_decl(DeclInfo {
                node_type: "struct",
                name: #name,
                visibility: #vis_str,
                module: #module_path_str,
                file: file!(), // Use file!() from the context where the macro is expanded
                line: #line,
                hash: #hash,
            });
        };
    };
    
    let output = quote! {
        #item
        #registration_code
    };
    
    output.into()
}

fn wrap_enum(args: &DeclArgs, item: &mut ItemEnum) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));
    
    let vis_str = match &item.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Restricted(_) => "pub(restricted)",
        syn::Visibility::Inherited => "private",
    };
    
    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();
    
    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            use introspector_decl_common::{DeclInfo, register_decl};
            register_decl(DeclInfo {
                node_type: "enum",
                name: #name,
                visibility: #vis_str,
                module: #module_path_str,
                file: file!(), // Use file!() from the context where the macro is expanded
                line: #line,
                hash: #hash,
            });
        };
    };
    
    let output = quote! {
        #item
        #registration_code
    };
    
    output.into()
}

fn wrap_trait(args: &DeclArgs, item: &mut ItemTrait) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));
    
    let vis_str = match &item.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Restricted(_) => "pub(restricted)",
        syn::Visibility::Inherited => "private",
    };
    
    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();
    
    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            use introspector_decl_common::{DeclInfo, register_decl};
            register_decl(DeclInfo {
                node_type: "trait",
                name: #name,
                visibility: #vis_str,
                module: #module_path_str,
                file: file!(), // Use file!() from the context where the macro is expanded
                line: #line,
                hash: #hash,
            });
        };
    };
    
    let output = quote! {
        #item
        #registration_code
    };
    
    output.into()
}