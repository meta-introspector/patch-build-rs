//! This module contains the `lru` macro for caching function results.
//!
//! The `#[lru]` attribute macro provides a way to cache the results of functions
//! that are marked as pure. This is particularly useful in `build.rs` scripts
//! where re-executing expensive computations unnecessarily can slow down builds.
//!
//! The caching mechanism works as follows:
//! 1. When a function annotated with `#[lru]` is called, its arguments are
//!    serialized and hashed to generate a unique cache key.
//! 2. The macro checks if a cached result exists for this key in a file-system
//!    based cache (JSON files).
//! 3. If a cached result is found, it's deserialized and returned immediately,
//!    bypassing the original function's execution.
//! 4. If no cached result is found, the original function is executed. Its
//!    return value is then serialized, stored in the cache, and returned.
//! 5. A simple in-memory LRU mechanism (based on call order) tracks usage,
//!    and when the cache capacity is exceeded, the least recently used entry
//!    (and its corresponding file) is removed.
//!
//! # Attributes
//!
//! - `capacity` (optional): An integer specifying the maximum number of entries
//!   to keep in the cache. Defaults to `10`. When the cache exceeds this
//!   capacity, the least recently used entry is evicted.
//! - `cache_dir` (optional): A string literal specifying a subdirectory within
//!   the system's cache directory (or a `.patch_build_rs_cache` in the current
//!   directory if the system cache directory cannot be determined). This is
//!   where the serialized cache files will be stored. Defaults to
//!   `"target/lru_cache"`.
//!
//! # Requirements for annotated functions
//!
//! - All function arguments must implement `serde::Serialize`.
//! - The function's return type must implement `serde::Serialize` and `serde::Deserialize<'de>`.
//!
//! # Example
//!
//! ```rust
//! use patch_build_rs_macros::lru;
//! use std::{fs, path::PathBuf};
//!
//! #[lru(capacity = 5, cache_dir = "my_custom_cache")]
//! fn expensive_computation(input: String, config_path: PathBuf) -> String {
//!     // Simulate a time-consuming operation, e.g., reading a large file,
//!     // performing complex calculations, or interacting with external tools.
//!     println!("Executing expensive_computation for input: {}", input);
//!     let file_content = fs::read_to_string(&config_path)
//!         .unwrap_or_else(|_| "default_config".to_string());
//!     format!("result_for_{}_{}", input, file_content)
//! }
//!
//! fn main() {
//!     let path = PathBuf::from("config.txt"); // Assume this file exists for the example
//!     let _ = expensive_computation("first_run".to_string(), path.clone());
//!     let _ = expensive_computation("first_run".to_string(), path.clone()); // This call will be cached
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit, Ident, LitStr};
use syn::parse::{AttributeArgs, NestedMeta}; // Corrected import for AttributeArgs and NestedMeta
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use directories::BaseDirs;
use serde::{Serialize, Deserialize};
use proc_macro2::Span; // Import Span from proc_macro2

// Define a static, thread-safe cache to manage LRU behavior across calls within the same process.
// The HashMap stores the cache key, and for each key, a tuple of (serialized_arguments, last_accessed_timestamp).
// The serialized_arguments are used to derive the cache key for file system storage.
pub static LRU_ACCESS_TRACKER: Lazy<Mutex<HashMap<String, u64>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Determines the root directory for all caches managed by `patch-build-rs-macros`.
/// It prioritizes the system's standard cache directory, falling back to a local
/// `.patch_build_rs_cache` if the system directory cannot be determined.
fn get_cache_root_dir() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        base_dirs.cache_dir().join("patch_build_rs_cache")
    } else {
        PathBuf::from("./.patch_build_rs_cache")
    }
}

/// Generates a unique SHA256 hash for any serializable input. This hash serves
/// as the cache key for both the in-memory tracker and the file-system storage.
pub fn get_cache_key<T: Serialize>(args: &T) -> String {
    let serialized = serde_json::to_string(args).expect("Failed to serialize arguments for caching");
    format!("{:x}", Sha256::digest(serialized.as_bytes()))
}

/// The `lru` attribute macro logic.
/// This function is intended to be called by a #[proc_macro_attribute] in lib.rs
/// to correctly handle macro expansion and meet proc-macro limitations.
pub fn lru_macro_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input_fn = parse_macro_input!(input as ItemFn);

    // Default values for capacity and cache_dir
    let mut capacity: usize = 10;
    let mut cache_dir_str = "target/lru_cache".to_string();

    // Parse macro arguments for `capacity` and `cache_dir`
    for arg in args {
        match arg {
            NestedMeta::Meta(syn::Meta::NameValue(m)) => {
                if m.path.is_ident("capacity") {
                    if let syn::Expr::Lit(expr_lit) = &m.value { // Corrected access to literal value
                        if let Lit::Int(lit) = &expr_lit.lit {
                            capacity = lit.base10_parse().expect("capacity must be an integer");
                        }
                    }
                } else if m.path.is_ident("cache_dir") {
                    if let syn::Expr::Lit(expr_lit) = &m.value { // Corrected access to literal value
                        if let Lit::Str(lit) = &expr_lit.lit {
                            cache_dir_str = lit.value();
                        }
                    }
                }
            }
            _ => panic!("Unsupported attribute argument. Expected `capacity = <int>` or `cache_dir = <string>`."),
        }
    }

    let fn_name = &input_fn.sig.ident;
    let visibility = &input_fn.vis;
    let generics = &input_fn.sig.generics;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;
    
    // Extract argument names to create a serializable struct for cache key generation.
    let input_names: Vec<Ident> = input_fn.sig.inputs.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(pat_ident.ident.clone());
            }
        }
        None
    }).collect();

    // Determine the return type for deserialization.
    let ret_type = match output {
        syn::ReturnType::Default => quote! { () }, // Handle functions that return nothing
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    // Construct the specific cache directory path for this function.
    let cache_dir_path_pb = get_cache_root_dir().join(&cache_dir_str).join(fn_name.to_string());
    let cache_dir_path_str = cache_dir_path_pb.to_string_lossy().to_string();
    let cache_dir_lit_str = LitStr::new(&cache_dir_path_str, Span::call_site());


    // The expanded code for the cached function.
    let expanded = quote! {
        #visibility fn #fn_name #generics(#inputs) #output {
            // A temporary struct to make function arguments serializable for hashing.
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            struct CacheableArgs<'a> {
                #(#input_names: &'a (impl serde::Serialize + ?Sized),)*
            }

            let args_for_key = CacheableArgs {
                #(#input_names: &#input_names),*
            };
            let cache_key = patch_build_rs_macros::lru_cache::get_cache_key(&args_for_key);
            let cache_file_path = std::path::PathBuf::from(#cache_dir_lit_str).join(&cache_key);

            // Update access time for LRU tracking.
            {
                let mut tracker = patch_build_rs_macros::lru_cache::LRU_ACCESS_TRACKER.lock().unwrap();
                tracker.insert(cache_key.clone(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            }

            // Attempt to load result from cache file.
            if cache_file_path.exists() {
                if let Ok(cached_data) = fs::read_to_string(&cache_file_path) {
                    if let Ok(result) = serde_json::from_str::<#ret_type>(&cached_data) {
                        return result; // Cache hit: return cached result.
                    } else {
                        // Cache file corrupted or incompatible, remove it to force re-computation
                        let _ = fs::remove_file(&cache_file_path);
                    }
                } else {
                    // Could not read file, remove it to force re-computation
                    let _ = fs::remove_file(&cache_file_path);
                }
            }

            // Cache miss: execute original function body.
            let result = #block;

            // Store the new result in the cache.
            if let Some(parent) = cache_file_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create cache directory for LRU macro.");
            }
            let serialized_result = serde_json::to_string(&result).expect("Failed to serialize function result for LRU macro caching.");
            fs::write(&cache_file_path, serialized_result).expect("Failed to write LRU macro cache file.");
            
            // Manage cache capacity (LRU eviction).
            {
                let mut tracker = patch_build_rs_macros::lru_cache::LRU_ACCESS_TRACKER.lock().unwrap();
                if tracker.len() > capacity {
                    // Find and remove the least recently used entry.
                    let mut oldest_key: Option<String> = None;
                    let mut oldest_time: u64 = std::u64::MAX;

                    for (key, &timestamp) in tracker.iter() {
                        if timestamp < oldest_time {
                            oldest_time = timestamp;
                            oldest_key = Some(key.clone());
                        }
                    }

                    if let Some(key_to_remove) = oldest_key {
                        tracker.remove(&key_to_remove);
                        // Also delete the actual cache file from the file system.
                        let file_to_remove = std::path::PathBuf::from(#cache_dir_lit_str).join(key_to_remove);
                        if file_to_remove.exists() {
                            let _ = fs::remove_file(file_to_remove);
                        }
                    }
                }
            }

            result // Return the computed result.
        }
    };

    expanded.into()
}
