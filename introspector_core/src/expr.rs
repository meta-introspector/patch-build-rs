/// The core inductive type for representing Rust expressions, similar to Lean4's `Expr`.
/// This allows for total reflection of the code into a manipulable data structure.
use crate::PureProgram;
use serde::{Deserialize, Serialize}; // Add serde imports
use syn::parse_str; // For parsing strings back to TokenStream
use std::hash::{Hash, Hasher}; // Add these for hashing
use std::collections::hash_map::DefaultHasher;
use crate::expr_cache::{EXPR_CACHE, SUBEXPR_COUNTS, SUBEXPR_LATTICE}; // Import caches

/// The core inductive type for representing Rust expressions, similar to Lean4's `Expr`.
/// This allows for total reflection of the code into a manipulable data structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] // Add Serialize, Deserialize
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

    // New variants for Rust language items, storing their TokenStream as String
    Function(String),
    Struct(String),
    Enum(String),
    Trait(String),
    Impl(String),
    Use(String),
    Module(String),
    Static(String),
    ConstItem(String), // Renamed from Const to avoid conflict
}

impl Expr {
    /// Helper to convert a syn item into its `proc_macro2::TokenStream` representation.
    /// This is used internally for the new `Expr` variants.
    fn item_to_tokenstream_string<T: quote::ToTokens>(item: T) -> String {
        let mut tokens = proc_macro2::TokenStream::new();
        item.to_tokens(&mut tokens);
        tokens.to_string() // Convert to String
    }

    /// Creates an `Expr::Function` from a `syn::ItemFn`.
    pub fn from_fn(item_fn: syn::ItemFn) -> Self {
        Expr::Function(Self::item_to_tokenstream_string(item_fn))
    }

    /// Creates an `Expr::Struct` from a `syn::ItemStruct`.
    pub fn from_struct(item_struct: syn::ItemStruct) -> Self {
        Expr::Struct(Self::item_to_tokenstream_string(item_struct))
    }

    /// Creates an `Expr::Enum` from a `syn::ItemEnum`.
    pub fn from_enum(item_enum: syn::ItemEnum) -> Self {
        Expr::Enum(Self::item_to_tokenstream_string(item_enum))
    }

    /// Creates an `Expr::Trait` from a `syn::ItemTrait`.
    pub fn from_trait(item_trait: syn::ItemTrait) -> Self {
        Expr::Trait(Self::item_to_tokenstream_string(item_trait))
    }

    /// Creates an `Expr::Impl` from a `syn::ItemImpl`.
    pub fn from_impl(item_impl: syn::ItemImpl) -> Self {
        Expr::Impl(Self::item_to_tokenstream_string(item_impl))
    }

    /// Creates an `Expr::Use` from a `syn::ItemUse`.
    pub fn from_use(item_use: syn::ItemUse) -> Self {
        Expr::Use(Self::item_to_tokenstream_string(item_use))
    }

    /// Creates an `Expr::Module` from a `syn::ItemMod`.
    pub fn from_module(item_mod: syn::ItemMod) -> Self {
        Expr::Module(Self::item_to_tokenstream_string(item_mod))
    }

    /// Creates an `Expr::Static` from a `syn::ItemStatic`.
    pub fn from_static(item_static: syn::ItemStatic) -> Self {
        Expr::Static(Self::item_to_tokenstream_string(item_static))
    }

    /// Creates an `Expr::ConstItem` from a `syn::ItemConst`.
    pub fn from_const_item(item_const: syn::ItemConst) -> Self {
        Expr::ConstItem(Self::item_to_tokenstream_string(item_const))
    }

    /// Hashes the current Expr and registers it, its sub-expressions,
    /// counts, and lattice relationships in the global caches.
    /// Returns the hash of the current Expr.
    pub fn hash_and_register_recursive(&self, parent_hash: Option<u64>) -> u64 {
        // 1. Calculate current Expr's hash (based on its serialized form)
        let expr_str = serde_json::to_string(self).expect("Failed to serialize Expr for hashing");
        let mut hasher = DefaultHasher::new();
        expr_str.hash(&mut hasher);
        let current_hash = hasher.finish();

        // 2. Add to EXPR_CACHE and SUBEXPR_COUNTS if not already present
        let mut cache_guard = EXPR_CACHE.lock().unwrap();
        let mut counts_guard = SUBEXPR_COUNTS.lock().unwrap();

        if cache_guard.get(&current_hash).is_none() {
            cache_guard.put(current_hash, (self.clone(), expr_str.clone()));
            counts_guard.insert(current_hash, 1);
        } else {
            // Increment count if already in cache
            *counts_guard.get_mut(&current_hash).unwrap() += 1;
        }

        // 3. Record lattice relationship
        if let Some(p_hash) = parent_hash {
            let mut lattice_guard = SUBEXPR_LATTICE.lock().unwrap();
            lattice_guard.entry(p_hash).or_insert_with(std::collections::BTreeSet::new).insert(current_hash);
        }

        // 4. Recursively process sub-expressions
        match self {
            Expr::Lam(_, body) => {
                body.hash_and_register_recursive(Some(current_hash));
            }
            Expr::App(func, args) => {
                func.hash_and_register_recursive(Some(current_hash));
                for arg in args {
                    arg.hash_and_register_recursive(Some(current_hash));
                }
            }
            // For variants holding String (originally TokenStream),
            // parse the string back to syn::Item and traverse it.
            // Simplified traversal for now: only direct Expr nesting in Lam/App are considered for lattice
            Expr::Function(_) | Expr::Struct(_) | Expr::Enum(_) |
            Expr::Trait(_) | Expr::Impl(_) | Expr::Use(_) |
            Expr::Module(_) | Expr::Static(_) | Expr::ConstItem(_) => {
                // No deep traversal into the syn::Item structure for now
            }
            _ => { /* No sub-expressions for Var, Const, PureAttractor directly */ }
        }

        current_hash
    }
}



