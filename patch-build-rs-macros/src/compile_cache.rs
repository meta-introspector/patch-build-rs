use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[decl(struct, name = "CompileError", vis = "pub", hash = "7a2f5c14")]
pub struct CompileError {
    pub file: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub error_code: Option<String>,
    pub timestamp: u64,
    pub fix_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[decl(struct, name = "CompileCache", vis = "pub", hash = "f0273978")]
pub struct CompileCache {
    pub errors: HashMap<String, CompileError>,
    pub successful_fixes: HashMap<String, String>, // error_hash -> fix_applied
}

impl CompileCache {
    pub fn load_or_create() -> Self {
        let cache_path = "target/compile_cache.json";
        
        if Path::new(cache_path).exists() {
            if let Ok(content) = fs::read_to_string(cache_path) {
                if let Ok(cache) = serde_json::from_str(&content) {
                    return cache;
                }
            }
        }
        
        CompileCache {
            errors: HashMap::new(),
            successful_fixes: HashMap::new(),
        }
    }
    
    pub fn save(&self) {
        fs::create_dir_all("target").ok();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            fs::write("target/compile_cache.json", content).ok();
        }
    }
    
    pub fn add_error(&mut self, error: CompileError) {
        let error_hash = self.hash_error(&error);
        self.errors.insert(error_hash, error);
        self.save();
    }
    
    pub fn get_error(&self, file: &str, message: &str) -> Option<&CompileError> {
        let temp_error = CompileError {
            file: file.to_string(),
            message: message.to_string(),
            line: None,
            column: None,
            error_code: None,
            timestamp: 0,
            fix_attempts: 0,
        };
        let hash = self.hash_error(&temp_error);
        self.errors.get(&hash)
    }
    
    fn hash_error(&self, error: &CompileError) -> String {
        format!("{}:{}", error.file, error.message)
    }
    
    pub fn should_auto_fix(&self, error: &CompileError) -> bool {
        error.fix_attempts < 3 // Max 3 auto-fix attempts
    }
}