use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;
use introspector_decl_core; // New dependency

// Attribute macro for wrapping declarations with metadata
#[proc_macro_attribute]
pub fn decl2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_2 = proc_macro2::TokenStream::from(attr);
    let item_2 = proc_macro2::TokenStream::from(item);
    introspector_decl_core::process_decl2_attribute_logic(attr_2, item_2).into()
}

// DECL_MODULE START
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro]
pub fn decl_module(input: TokenStream) -> TokenStream {
    let modules = syn::parse_macro_input!(input with Punctuated::<Ident, Comma>::parse_terminated);

    let mut generated_macros = proc_macro2::TokenStream::new();
    let mut module_names: Vec<String> = Vec::new();

    for module_ident in modules {
        let module_name_str = module_ident.to_string();
        module_names.push(module_name_str.clone());
        let macro_name = Ident::new(&format!("decl_{}", module_name_str), module_ident.span());

        let module_specific_boilerplate = Ident::new(&format!("{}_DEFAULT_MACRO_SET_DEFINED", module_name_str.to_uppercase()), module_ident.span());

        generated_macros.extend(quote! {
            #[proc_macro_attribute]
            pub fn #macro_name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let attr_2 = proc_macro2::TokenStream::from(attr);
                let item_2 = proc_macro2::TokenStream::from(item);
                let processed_item = introspector_decl_core::process_decl2_attribute_logic(attr_2, item_2);
                let additional_declarations = quote!{
                    #[allow(dead_code)]
                    const #module_specific_boilerplate: bool = true;
                };
                let mut final_output = proc_macro2::TokenStream::new();
                final_output.extend(proc_macro2::TokenStream::from(processed_item));
                final_output.extend(additional_declarations);
                final_output.into()
            }
        });
    }

    let module_names_str: Vec<String> = module_names.iter().map(|s| s.to_string()).collect();

    // Generate modules_list! macro
    let modules_list_output = quote! {
        #[macro_export]
        macro_rules! modules_list {
            () => {
                &[ #(#module_names_str),* ]
            };
        }
    };
    generated_macros.extend(modules_list_output);

    // Placeholder for module_members! macro (conceptual)
    let module_members_output = quote! {
        #[macro_export]
        macro_rules! module_members {
            ($module_name:expr) => {
                // This would eventually query introspector_decl_common for members of $module_name
                // For now, it's a placeholder.
                compile_error!("module_members! is not yet implemented to query actual members.");
            };
        }
    };
    generated_macros.extend(module_members_output);

    // Generate prelude module
    let all_decl_macros_re_exports = module_names.iter().map(|module_name_str| {
        let macro_name = Ident::new(&format!("decl_{}", module_name_str), proc_macro2::Span::call_site());
        quote! {
            pub use super::#macro_name;
        }
    });

    let prelude_module = quote! {
        #[macro_export]
        mod prelude {
            pub use introspector_decl_common::{DeclInfo, register_decl};
            #(#all_decl_macros_re_exports)*
        }
    };
    generated_macros.extend(prelude_module);

    generated_macros.into()
}
