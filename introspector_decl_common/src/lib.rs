use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

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