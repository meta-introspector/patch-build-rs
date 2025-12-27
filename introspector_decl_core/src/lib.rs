use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    ItemFn, ItemStruct, ItemEnum, ItemTrait, Ident,
    Visibility, LitStr, parse::Parse, parse::ParseStream, Result as SynResult
};
use introspector_macro_helpers::{generate_decl_registration, parse_decl_args, dispatch_wrap_logic};

// Core logic function
pub fn process_decl2_attribute_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_decl_args!(attr);
    dispatch_wrap_logic!(item, args)
}


pub struct DeclArgs {
    pub node_type: Option<String>,
    pub name: Option<String>,
    pub vis: Option<String>,
    pub hash: Option<String>,
    pub extra: Vec<(String, String)>,
}

impl Parse for DeclArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
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
            let value: LitStr = input.parse()?;

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

pub fn compute_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:08x}", hasher.finish() & 0xFFFFFFFF)
}

pub fn wrap_function(args: &DeclArgs, item: &mut ItemFn) -> TokenStream {
    let name = item.sig.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));

    let vis_str = match &item.vis {
        Visibility::Public(_) => "pub",
        Visibility::Restricted(_) => "pub(restricted)",
        Visibility::Inherited => "private",
        _ => "unknown_visibility",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();

    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            introspector_macro_helpers::generate_decl_registration!(
                "fn",
                #name,
                #vis_str,
                #module_path_str,
                file!(),
                #line,
                #hash
            );
        };
    };

    let output = quote! {
        #item
        #registration_code
    };

    output.into()
}

pub fn wrap_struct(args: &DeclArgs, item: &mut ItemStruct) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));

    let vis_str = match &item.vis {
        Visibility::Public(_) => "pub",
        Visibility::Restricted(_) => "pub(restricted)",
        Visibility::Inherited => "private",
        _ => "unknown_visibility",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();

    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            introspector_macro_helpers::generate_decl_registration!(
                "struct",
                #name,
                #vis_str,
                #module_path_str,
                file!(),
                #line,
                #hash
            );
        };
    };

    let output = quote! {
        #item
        #registration_code
    };

    output.into()
}

pub fn wrap_enum(args: &DeclArgs, item: &mut ItemEnum) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));

    let vis_str = match &item.vis {
        Visibility::Public(_) => "pub",
        Visibility::Restricted(_) => "pub(restricted)",
        Visibility::Inherited => "private",
        _ => "unknown_visibility",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();

    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            introspector_macro_helpers::generate_decl_registration!(
                "enum",
                #name,
                #vis_str,
                #module_path_str,
                file!(),
                #line,
                #hash
            );
        };
    };

    let output = quote! {
        #item
        #registration_code
    };

    output.into()
}

pub fn wrap_trait(args: &DeclArgs, item: &mut ItemTrait) -> TokenStream {
    let name = item.ident.to_string();
    let hash = args.hash.clone().unwrap_or_else(|| compute_hash(&item.to_token_stream().to_string()));

    let vis_str = match &item.vis {
        Visibility::Public(_) => "pub",
        Visibility::Restricted(_) => "pub(restricted)",
        Visibility::Inherited => "private",
        _ => "unknown_visibility",
    };

    let line = 0 as u32; // Temporary workaround for proc_macro2::Span::start().line issue
    let module_path_str = module_path!().to_string();

    let registration_code = quote! {
        // Auto-generated declaration registration
        const _: () = {
            introspector_macro_helpers::generate_decl_registration!(
                "trait",
                #name,
                #vis_str,
                #module_path_str,
                file!(),
                #line,
                #hash
            );
        };
    };

    let output = quote! {
        #item
        #registration_code
    };

    output.into()
}
