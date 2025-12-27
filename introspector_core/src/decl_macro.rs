use std::collections::HashMap;
use introspector_decl_common::{get_all_declarations, DeclInfo};

#[decl(fn, name = "print_declaration_summary", vis = "pub", hash = "5990104a")]
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

#[decl(fn, name = "generate_rdf_declarations", vis = "pub", hash = "aa5f62fe")]
pub fn generate_rdf_declarations() -> String {
    let decls = get_all_declarations();
    let mut rdf = String::new();
    
    rdf.push_str("@prefix decl: <http://patch-build-rs.local/decl/> .\n");
    rdf.push_str("@prefix rust: <http://rust-lang.org/> .\n");
    rdf.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n");
    
    for decl in &decls {
        rdf.push_str(&format!("decl:{0} a rust:{1} ;\n", decl.hash, decl.node_type));
        rdf.push_str(&format!("    decl:name \"{0}\" ;\n", decl.name));
        rdf.push_str(&format!("    decl:visibility \"{0}\" ;\n", decl.visibility));
        rdf.push_str(&format!("    decl:module \"{0}\" ;\n", decl.module));
        rdf.push_str(&format!("    decl:file \"{0}\" ;\n", decl.file));
        rdf.push_str(&format!("    decl:line \"{0}\"^^xsd:integer .\n\n", decl.line));
    }
    
    rdf
}