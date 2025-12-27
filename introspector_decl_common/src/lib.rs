use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use lru::LruCache; // New import
use std::num::NonZeroUsize; // New import

#[derive(Debug, Clone)]
pub struct DeclInfo {
    pub node_type: &'static str,
    pub name: &'static str,
    pub visibility: &'static str,
    pub module: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub hash: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct DeclRegistry {
    pub declarations: Vec<DeclInfo>,
    pub by_type: HashMap<String, Vec<usize>>,
    pub by_module: HashMap<String, Vec<usize>>,
    pub by_hash: HashMap<String, usize>,
}

pub static DECL_REGISTRY: Lazy<Mutex<DeclRegistry>> = 
    Lazy::new(|| Mutex::new(DeclRegistry::default()));

pub fn register_decl(info: DeclInfo) {
    if let Ok(mut registry) = DECL_REGISTRY.lock() {
        let idx = registry.declarations.len();
        
        registry.by_type
            .entry(info.node_type.to_string())
            .or_default()
            .push(idx);
        
        registry.by_module
            .entry(info.module.to_string())
            .or_default()
            .push(idx);
        
        registry.by_hash
            .insert(info.hash.to_string(), idx);
        
        registry.declarations.push(info);
    }
}

pub fn get_all_declarations() -> Vec<DeclInfo> {
    DECL_REGISTRY.lock()
        .map(|r| r.declarations.clone())
        .unwrap_or_default()
}

pub fn get_declarations_by_type(node_type: &str) -> Vec<DeclInfo> {
    DECL_REGISTRY.lock()
        .map(|r| {
            r.by_type.get(node_type)
                .map(|indices| {
                    indices.iter()
                        .filter_map(|&i| r.declarations.get(i).cloned())
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

pub fn get_declarations_by_module(module: &str) -> Vec<DeclInfo> {
    DECL_REGISTRY.lock()
        .map(|r| {
            r.by_module.get(module)
                .map(|indices| {
                    indices.iter()
                        .filter_map(|&i| r.declarations.get(i).cloned())
                        .collect()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

pub fn get_declaration_by_hash(hash: &str) -> Option<DeclInfo> {
    DECL_REGISTRY.lock().ok()?.by_hash.get(hash)
        .and_then(|&idx| {
            DECL_REGISTRY.lock().ok()?.declarations.get(idx).cloned()
        })
}

// Global LRU Cache for storing arbitrary data (e.g., serialized objects)
pub static LRU_CACHE: Lazy<Mutex<LruCache<String, Vec<u8>>>> = Lazy::new(|| {
    // Set a default capacity for the LRU cache
    let capacity = NonZeroUsize::new(1024).expect("Cache capacity must be non-zero");
    Mutex::new(LruCache::new(capacity))
});

pub fn store_in_lru_cache(key: String, value: Vec<u8>) {
    if let Ok(mut cache) = LRU_CACHE.lock() {
        cache.put(key, value);
    }
}

pub fn query_from_lru_cache(key: &str) -> Option<Vec<u8>> {
    if let Ok(mut cache) = LRU_CACHE.lock() {
        cache.get(key).cloned()
    } else {
        None
    }
}
