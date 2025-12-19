/// The core inductive type for representing Rust expressions, similar to Lean4's `Expr`.
/// This allows for total reflection of the code into a manipulable data structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A variable, identified by name.
    Var(String),
    /// A constant, identified by name.
    Const(String),
    /// A lambda abstraction (e.g., `fun x => body`).
    Lam(String, Box<Expr>),
    /// A function application (e.g., `f a b`).
    App(Box<Expr>, Vec<Expr>),
    /// A bridge to the numeric encoding of a Rust program from Layer 14.
    PureAttractor(PureProgram),

    // New variants for Rust language items, storing their TokenStream
    Function(proc_macro2::TokenStream),
    Struct(proc_macro2::TokenStream),
    Enum(proc_macro2::TokenStream),
    Trait(proc_macro2::TokenStream),
    Impl(proc_macro2::TokenStream),
    Use(proc_macro2::TokenStream),
    Module(proc_macro2::TokenStream),
    Static(proc_macro2::TokenStream),
    ConstItem(proc_macro2::TokenStream), // Renamed from Const to avoid conflict
}

impl Expr {
    /// Helper to convert a syn item into its `proc_macro2::TokenStream` representation.
    /// This is used internally for the new `Expr` variants.
    fn item_to_tokenstream<T: quote::ToTokens>(item: T) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        item.to_tokens(&mut tokens);
        tokens
    }

    /// Creates an `Expr::Function` from a `syn::ItemFn`.
    pub fn from_fn(item_fn: syn::ItemFn) -> Self {
        Expr::Function(Self::item_to_tokenstream(item_fn))
    }

    /// Creates an `Expr::Struct` from a `syn::ItemStruct`.
    pub fn from_struct(item_struct: syn::ItemStruct) -> Self {
        Expr::Struct(Self::item_to_tokenstream(item_struct))
    }

    /// Creates an `Expr::Enum` from a `syn::ItemEnum`.
    pub fn from_enum(item_enum: syn::ItemEnum) -> Self {
        Expr::Enum(Self::item_to_tokenstream(item_enum))
    }

    /// Creates an `Expr::Trait` from a `syn::ItemTrait`.
    pub fn from_trait(item_trait: syn::ItemTrait) -> Self {
        Expr::Trait(Self::item_to_tokenstream(item_trait))
    }

    /// Creates an `Expr::Impl` from a `syn::ItemImpl`.
    pub fn from_impl(item_impl: syn::ItemImpl) -> Self {
        Expr::Impl(Self::item_to_tokenstream(item_impl))
    }

    /// Creates an `Expr::Use` from a `syn::ItemUse`.
    pub fn from_use(item_use: syn::ItemUse) -> Self {
        Expr::Use(Self::item_to_tokenstream(item_use))
    }

    /// Creates an `Expr::Module` from a `syn::ItemMod`.
    pub fn from_module(item_mod: syn::ItemMod) -> Self {
        Expr::Module(Self::item_to_tokenstream(item_mod))
    }

    /// Creates an `Expr::Static` from a `syn::ItemStatic`.
    pub fn from_static(item_static: syn::ItemStatic) -> Self {
        Expr::Static(Self::item_to_tokenstream(item_static))
    }

    /// Creates an `Expr::ConstItem` from a `syn::ItemConst`.
    pub fn from_const_item(item_const: syn::ItemConst) -> Self {
        Expr::ConstItem(Self::item_to_tokenstream(item_const))
    }
}

impl quote::ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote; // Import quote macro locally
        

        match self {
            Expr::Var(s) => tokens.extend(quote! { introspector_core::Expr::Var(#s.to_string()) }),
            Expr::Const(s) => tokens.extend(quote! { introspector_core::Expr::Const(#s.to_string()) }),
            Expr::Lam(s, body) => tokens.extend(quote! { introspector_core::Expr::Lam(#s.to_string(), Box::new(#body)) }),
            Expr::App(f, args) => tokens.extend(quote! { introspector_core::Expr::App(Box::new(#f), vec![#(#args),*]) }),
            Expr::PureAttractor(pure_program) => {
                let set_values = pure_program.set.iter().collect::<Vec<_>>();
                let name = &pure_program.name;
                tokens.extend(quote! {
                    introspector_core::Expr::PureAttractor(
                        introspector_core::PureProgram {
                            set: std::collections::BTreeSet::from_iter(vec![#(#set_values),*]),
                            name: #name.to_string(),
                        }
                    )
                })
            }
            Expr::Function(ts) => tokens.extend(quote! { introspector_core::Expr::Function(#ts) }),
            Expr::Struct(ts) => tokens.extend(quote! { introspector_core::Expr::Struct(#ts) }),
            Expr::Enum(ts) => tokens.extend(quote! { introspector_core::Expr::Enum(#ts) }),
            Expr::Trait(ts) => tokens.extend(quote! { introspector_core::Expr::Trait(#ts) }),
            Expr::Impl(ts) => tokens.extend(quote! { introspector_core::Expr::Impl(#ts) }),
            Expr::Use(ts) => tokens.extend(quote! { introspector_core::Expr::Use(#ts) }),
            Expr::Module(ts) => tokens.extend(quote! { introspector_core::Expr::Module(#ts) }),
            Expr::Static(ts) => tokens.extend(quote! { introspector_core::Expr::Static(#ts) }),
            Expr::ConstItem(ts) => tokens.extend(quote! { introspector_core::Expr::ConstItem(#ts) }),
        }
    }
}

