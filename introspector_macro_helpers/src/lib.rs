#[macro_export]
macro_rules! generate_decl_registration {
    (
        $node_type:expr,
        $name_expr:expr,
        $vis_expr:expr,
        $module_path_expr:expr,
        $file_expr:expr,
        $line_expr:expr,
        $hash_expr:expr
    ) => {
        {
            use introspector_decl_common::{DeclInfo, register_decl};
            register_decl(DeclInfo {
                node_type: $node_type,
                name: $name_expr,
                visibility: $vis_expr,
                module: $module_path_expr,
                file: $file_expr,
                line: $line_expr,
                hash: $hash_expr,
            });
        }
    };
}

#[macro_export]
macro_rules! is_attribute_ident {
    ($attr:expr, $ident_str:expr) => {
        $attr.path().is_ident($ident_str)
    };
}

#[macro_export]
macro_rules! is_use_decl_module {
    ($item_use:expr) => {
        if let syn::UseTree::Path(use_tree_path) = &$item_use.tree {
            if use_tree_path.ident == "introspector_decl2_macros" {
                if let syn::UseTree::Name(use_tree_name) = &*use_tree_path.tree {
                    use_tree_name.ident == "decl_module"
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };
}

// New macros to append
#[macro_export]
macro_rules! parse_decl_args {
    ($attr_token_stream:expr) => {
        syn::parse2($attr_token_stream).unwrap() // Consider better error handling later
    };
}

#[macro_export]
macro_rules! dispatch_wrap_logic {
    ($item_token_stream:expr, $args_struct:expr) => {
        {
            let item_clone = $item_token_stream.clone();
            let args = $args_struct;

            if let Ok(mut item_fn) = syn::parse2::<syn::ItemFn>(item_clone.clone()) {
                crate::wrap_function(&args, &mut item_fn)
            } else if let Ok(mut item_struct) = syn::parse2::<syn::ItemStruct>(item_clone.clone()) {
                crate::wrap_struct(&args, &mut item_struct)
            } else if let Ok(mut item_enum) = syn::parse2::<syn::ItemEnum>(item_clone.clone()) {
                crate::wrap_enum(&args, &mut item_enum)
            } else if let Ok(mut item_trait) = syn::parse2::<syn::ItemTrait>(item_clone.clone()) {
                crate::wrap_trait(&args, &mut item_trait)
            } else {
                $item_token_stream // If we can't parse, just return the original
            }
        }
    };
}
