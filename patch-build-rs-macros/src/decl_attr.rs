use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, ItemFn, ItemStruct, ItemEnum, ItemTrait,
    ItemConst, ItemType, Attribute, Meta, MetaList, NestedMeta, Lit, Ident,
};

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
    
    let params: Vec<String> = item.sig.inputs.iter().map(|arg| {
        arg.to_token_stream().to_string()
    }).collect();
    let params_str = params.join(", ");
    
    let return_type = match &item.sig.output {
        syn::ReturnType::Default => "()".to_string(),
        syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
    };
    
    // Generate the registration code
    let name_lit = syn::LitStr::new(&name, proc_macro2::Span::call_site());
    let hash_lit = syn::LitStr::new(&hash, proc_macro2::Span::call_site());
    let vis_lit = syn::LitStr::new(vis_str, proc_macro2::Span::call_site());
    let params_lit = syn::LitStr::new(&params_str, proc_macro2::Span::call_site());
    let ret_lit = syn::LitStr::new(&return_type, proc_macro2::Span::call_site());
    
    let output = quote! {
        #item
        
        // Auto-generated declaration registration
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".decl_registry"]
            static __DECL_REG: &[u8] = concat!(
                "DECL:fn:", #name_lit, ":", #vis_lit, ":", #hash_lit, ":", #params_lit, " -> ", #ret_lit
            ).as_bytes();
        };
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
    
    let field_count = match &item.fields {
        syn::Fields::Named(f) => f.named.len(),
        syn::Fields::Unnamed(f) => f.unnamed.len(),
        syn::Fields::Unit => 0,
    };
    
    let name_lit = syn::LitStr::new(&name, proc_macro2::Span::call_site());
    let hash_lit = syn::LitStr::new(&hash, proc_macro2::Span::call_site());
    let vis_lit = syn::LitStr::new(vis_str, proc_macro2::Span::call_site());
    let fields_lit = syn::LitStr::new(&field_count.to_string(), proc_macro2::Span::call_site());
    
    let output = quote! {
        #item
        
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".decl_registry"]
            static __DECL_REG: &[u8] = concat!(
                "DECL:struct:", #name_lit, ":", #vis_lit, ":", #hash_lit, ":fields=", #fields_lit
            ).as_bytes();
        };
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
    
    let variants: Vec<String> = item.variants.iter()
        .map(|v| v.ident.to_string())
        .collect();
    let variants_str = variants.join(",");
    
    let name_lit = syn::LitStr::new(&name, proc_macro2::Span::call_site());
    let hash_lit = syn::LitStr::new(&hash, proc_macro2::Span::call_site());
    let vis_lit = syn::LitStr::new(vis_str, proc_macro2::Span::call_site());
    let variants_lit = syn::LitStr::new(&variants_str, proc_macro2::Span::call_site());
    
    let output = quote! {
        #item
        
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".decl_registry"]
            static __DECL_REG: &[u8] = concat!(
                "DECL:enum:", #name_lit, ":", #vis_lit, ":", #hash_lit, ":variants=", #variants_lit
            ).as_bytes();
        };
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
    
    let methods: Vec<String> = item.items.iter()
        .filter_map(|i| {
            if let syn::TraitItem::Fn(m) = i {
                Some(m.sig.ident.to_string())
            } else {
                None
            }
        })
        .collect();
    let methods_str = methods.join(",");
    
    let name_lit = syn::LitStr::new(&name, proc_macro2::Span::call_site());
    let hash_lit = syn::LitStr::new(&hash, proc_macro2::Span::call_site());
    let vis_lit = syn::LitStr::new(vis_str, proc_macro2::Span::call_site());
    let methods_lit = syn::LitStr::new(&methods_str, proc_macro2::Span::call_site());
    
    let output = quote! {
        #item
        
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[doc(hidden)]
            #[link_section = ".decl_registry"]
            static __DECL_REG: &[u8] = concat!(
                "DECL:trait:", #name_lit, ":", #vis_lit, ":", #hash_lit, ":methods=", #methods_lit
            ).as_bytes();
        };
    };
    
    output.into()
}
