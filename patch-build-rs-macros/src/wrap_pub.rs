use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Item, ItemConst, ItemEnum, ItemFn, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemType, Visibility};

#[proc_macro_attribute]
pub fn wrap_pub_items(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            attr,
            "#[wrap_pub_items] currently does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }

    let mut input_module = parse_macro_input!(item as ItemMod);

    if let Some((_, items)) = &mut input_module.content {
        for item in items.iter_mut() {
            let is_public = match item {
                Item::Const(item_const) => matches!(item_const.vis, Visibility::Public(_)),
                Item::Enum(item_enum) => matches!(item_enum.vis, Visibility::Public(_)),
                Item::Fn(item_fn) => matches!(item_fn.vis, Visibility::Public(_)),
                Item::Mod(item_mod) => matches!(item_mod.vis, Visibility::Public(_)),
                Item::Static(item_static) => matches!(item_static.vis, Visibility::Public(_)),
                Item::Struct(item_struct) => matches!(item_struct.vis, Visibility::Public(_)),
                Item::Trait(item_trait) => matches!(item_trait.vis, Visibility::Public(_)),
                Item::Type(item_type) => matches!(item_type.vis, Visibility::Public(_)),
                _ => false, // Other items are not typically 'pub' in the same way, or can't be wrapped easily.
            };

            if is_public {
                // Prepend a new attribute macro to the public item
                // This assumes `__my_public_item_wrapper_attr` will be defined elsewhere as an attribute macro.
                let new_attr: Attribute = syn::parse_quote! { #[__my_public_item_wrapper_attr] };

                match item {
                    Item::Const(item_const) => item_const.attrs.insert(0, new_attr),
                    Item::Enum(item_enum) => item_enum.attrs.insert(0, new_attr),
                    Item::Fn(item_fn) => item_fn.attrs.insert(0, new_attr),
                    Item::Mod(item_mod) => item_mod.attrs.insert(0, new_attr),
                    Item::Static(item_static) => item_static.attrs.insert(0, new_attr),
                    Item::Struct(item_struct) => item_struct.attrs.insert(0, new_attr),
                    Item::Trait(item_trait) => item_trait.attrs.insert(0, new_attr),
                    Item::Type(item_type) => item_type.attrs.insert(0, new_attr),
                    _ => {}, // Should not happen given the 'is_public' check, but good for completeness.
                }
            }
        }
    }

    quote! { #input_module }.into()
}
