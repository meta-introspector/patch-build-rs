use once_cell::sync::Lazy;
use std::sync::Mutex;
use lru::LruCache;
use crate::Expr;
use serde_json;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::collections::BTreeSet; // Add this import

pub static EXPR_CACHE: Lazy<Mutex<LruCache<u64, (Expr, String)>>> = Lazy::new(|| {
    Mutex::new(LruCache::new(std::num::NonZeroUsize::new(1024).unwrap())) // Cache capacity of 1024
});

// Global static HashMap to store counts of subexpression hashes
pub static SUBEXPR_COUNTS: Lazy<Mutex<HashMap<u64, usize>>> = Lazy::new(|| { // Made public
    Mutex::new(HashMap::new())
});

// Global static HashMap to store the subexpression lattice
// Key: Hash of the parent Expr
// Value: BTreeSet of hashes of Exprs directly contained by the parent
pub static SUBEXPR_LATTICE: Lazy<Mutex<HashMap<u64, BTreeSet<u64>>>> = Lazy::new(|| { // Made public
    Mutex::new(HashMap::new())
});

// Function to save the EXPR_CACHE to a JSON file
#[decl(fn, name = "write_cache_to_json", vis = "pub", hash = "d3657087")]
pub fn write_cache_to_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cache_guard = EXPR_CACHE.lock().unwrap();
    
    // Convert LruCache to a serializable structure.
    // LruCache itself is not directly serializable.
    // We'll convert it to a Vec of its key-value pairs.
    let serializable_cache: Vec<(u64, (Expr, String))> = cache_guard
        .iter()
        .map(|(key, value)| (*key, value.clone()))
        .collect();

    let json_string = serde_json::to_string_pretty(&serializable_cache)?;
    fs::write(file_path, json_string)?;
    Ok(())
}

// Function to load the EXPR_CACHE from a JSON file
#[decl(fn, name = "load_cache_from_json", vis = "pub", hash = "319d2a2d")]
pub fn load_cache_from_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if file_path.exists() {
        let json_string = fs::read_to_string(file_path)?;
        let deserialized_cache: Vec<(u64, (Expr, String))> = serde_json::from_str(&json_string)?;
        
        let mut cache_guard = EXPR_CACHE.lock().unwrap();
        cache_guard.clear(); // Clear existing cache before loading
        for (key, value) in deserialized_cache {
            cache_guard.put(key, value);
        }
    }
    Ok(())
}

// Function to get a clone of the current subexpression counts
#[decl(fn, name = "get_subexpr_counts", vis = "pub", hash = "6cb7331a")]
pub fn get_subexpr_counts() -> HashMap<u64, usize> {
    SUBEXPR_COUNTS.lock().unwrap().clone()
}

// Function to save the SUBEXPR_COUNTS to a JSON file
#[decl(fn, name = "write_counts_to_json", vis = "pub", hash = "3beef4da")]
pub fn write_counts_to_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let counts_guard = SUBEXPR_COUNTS.lock().unwrap();
    let json_string = serde_json::to_string_pretty(&*counts_guard)?;
    fs::write(file_path, json_string)?;
    Ok(())
}

// Function to load SUBEXPR_COUNTS from a JSON file
#[decl(fn, name = "load_counts_from_json", vis = "pub", hash = "4e14e092")]
pub fn load_counts_from_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if file_path.exists() {
        let json_string = fs::read_to_string(file_path)?;
        let deserialized_counts: HashMap<u64, usize> = serde_json::from_str(&json_string)?;
        
        let mut counts_guard = SUBEXPR_COUNTS.lock().unwrap();
        counts_guard.clear(); // Clear existing counts before loading
        for (key, value) in deserialized_counts {
            counts_guard.insert(key, value);
        }
    }
    Ok(())
}

// Function to get a clone of the current subexpression lattice
#[decl(fn, name = "get_subexpr_lattice", vis = "pub", hash = "2f65e723")]
pub fn get_subexpr_lattice() -> HashMap<u64, BTreeSet<u64>> {
    SUBEXPR_LATTICE.lock().unwrap().clone()
}

// Function to save the SUBEXPR_LATTICE to a JSON file
#[decl(fn, name = "write_lattice_to_json", vis = "pub", hash = "5d905aef")]
pub fn write_lattice_to_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let lattice_guard = SUBEXPR_LATTICE.lock().unwrap();
    let json_string = serde_json::to_string_pretty(&*lattice_guard)?;
    fs::write(file_path, json_string)?;
    Ok(())
}

// Function to load SUBEXPR_LATTICE from a JSON file
#[decl(fn, name = "load_lattice_from_json", vis = "pub", hash = "98a83f90")]
pub fn load_lattice_from_json(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if file_path.exists() {
        let json_string = fs::read_to_string(file_path)?;
        let deserialized_lattice: HashMap<u64, BTreeSet<u64>> = serde_json::from_str(&json_string)?;
        
        let mut lattice_guard = SUBEXPR_LATTICE.lock().unwrap();
        lattice_guard.clear(); // Clear existing lattice before loading
        for (key, value) in deserialized_lattice {
            lattice_guard.insert(key, value);
        }
    }
    Ok(())
}