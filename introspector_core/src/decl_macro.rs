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

pub fn print_declaration_summary() {
    let decls = get_all_declarations();
    let reset = "\x1b[0m";
    let cyan = "\x1b[96m";
    let green = "\x1b[92m";
    
    eprintln!("\n{}╔═══════════════════════════════════════════════════════════════╗{}", cyan, reset);
    eprintln!("{}║              📋 DECLARATION REGISTRY                          ║{}", cyan, reset);
    eprintln!("{}╚═══════════════════════════════════════════════════════════════╝{}\n", cyan, reset);
    
    // Count by type
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for decl in &decls {
        *type_counts.entry(decl.node_type).or_insert(0) += 1;
    }
    
    eprintln!("{}📊 By Type:{}", green, reset);
    for (ty, count) in &type_counts {
        let emoji = match *ty {
            "fn" => "🔧",
            "struct" => "📦",
            "enum" => "🔀",
            "trait" => "🎭",
            "impl" => "⚙️",
            "mod" => "📁",
            _ => "📄",
        };
        eprintln!("   {} {}: {}", emoji, ty, count);
    }
    
    eprintln!("\n{}📋 All Declarations ({}):{}", cyan, decls.len(), reset);
    eprintln!("{:<8} {:<6} {:<30} {:<20} {}", "Type", "Vis", "Name", "Module", "Hash");
    eprintln!("{}", "─".repeat(80));
    
    for decl in decls.iter().take(50) {
        eprintln!("{:<8} {:<6} {:<30} {:<20} {}", 
            decl.node_type,
            decl.visibility,
            decl.name.chars().take(28).collect::<String>(),
            decl.module.split("::").last().unwrap_or(decl.module).chars().take(18).collect::<String>(),
            &decl.hash[..8]);
    }
    
    if decls.len() > 50 {
        eprintln!("... and {} more", decls.len() - 50);
    }
}

pub fn generate_rdf_declarations() -> String {
    let decls = get_all_declarations();
    let mut rdf = String::new();
    
    rdf.push_str("@prefix decl: <http://patch-build-rs.local/decl/> .\n");
    rdf.push_str("@prefix rust: <http://rust-lang.org/> .\n");
    rdf.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n");
    
    for decl in &decls {
        rdf.push_str(&format!("decl:{} a rust:{} ;\n", decl.hash, decl.node_type));
        rdf.push_str(&format!("    decl:name \"{}\" ;\n", decl.name));
        rdf.push_str(&format!("    decl:visibility \"{}\" ;\n", decl.visibility));
        rdf.push_str(&format!("    decl:module \"{}\" ;\n", decl.module));
        rdf.push_str(&format!("    decl:file \"{}\" ;\n", decl.file));
        rdf.push_str(&format!("    decl:line \"{}\"^^xsd:integer .\n\n", decl.line));
    }
    
    rdf
}

#[macro_export]
macro_rules! decl {
    (
        node_type: $type:literal,
        name: $name:literal,
        visibility: $vis:literal,
        line: $line:expr,
        generics: $generics:expr,
        hash: $hash:literal,
        fields: $fields:tt,
        variants: $variants:tt,
        methods: $methods:tt,
        doc: $doc:expr,
    ) => {
        $crate::decl_macro::register_decl($crate::decl_macro::DeclInfo {
            node_type: $type,
            name: $name,
            visibility: $vis,
            module: module_path!(),
            file: file!(),
            line: $line as u32,
            hash: $hash,
        });
    };
    
    // Simple form
    ($type:ident, $name:ident) => {
        $crate::decl_macro::register_decl($crate::decl_macro::DeclInfo {
            node_type: stringify!($type),
            name: stringify!($name),
            visibility: "pub",
            module: module_path!(),
            file: file!(),
            line: line!(),
            hash: concat!(stringify!($type), "_", stringify!($name)),
        });
    };
}

#[macro_export]
macro_rules! decl_fn {
    ($name:ident) => {
        $crate::decl!(fn, $name);
    };
}

#[macro_export]
macro_rules! decl_struct {
    ($name:ident) => {
        $crate::decl!(struct, $name);
    };
}

#[macro_export]
macro_rules! decl_enum {
    ($name:ident) => {
        $crate::decl!(enum, $name);
    };
}

#[macro_export]
macro_rules! decl_trait {
    ($name:ident) => {
        $crate::decl!(trait, $name);
    };
}

#[macro_export]
macro_rules! print_decls {
    () => {
        $crate::decl_macro::print_declaration_summary();
    };
}

#[macro_export]
macro_rules! decl_rdf {
    () => {
        $crate::decl_macro::generate_rdf_declarations()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_decl() {
        register_decl(DeclInfo {
            node_type: "fn",
            name: "test_function",
            visibility: "pub",
            module: "test_module",
            file: "test.rs",
            line: 10,
            hash: "abc123",
        });
        
        let decls = get_all_declarations();
        assert!(decls.iter().any(|d| d.name == "test_function"));
    }

    #[test]
    fn test_get_by_type() {
        register_decl(DeclInfo {
            node_type: "struct",
            name: "TestStruct",
            visibility: "pub",
            module: "test_module",
            file: "test.rs",
            line: 20,
            hash: "def456",
        });
        
        let structs = get_declarations_by_type("struct");
        assert!(structs.iter().any(|d| d.name == "TestStruct"));
    }
}
