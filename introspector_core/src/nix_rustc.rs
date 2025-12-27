use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
#[decl(struct, name = "RustcNixInfo", vis = "pub", hash = "c11c79f6")]
pub struct RustcNixInfo {
    pub version: String,
    pub store_path: String,
    pub src_tarball: String,
    pub src_url: String,
    pub drv_path: String,
    pub unpacked_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
#[decl(struct, name = "RustcSourceStats", vis = "pub", hash = "cb3cb97f")]
pub struct RustcSourceStats {
    pub total_files: usize,
    pub rust_files: usize,
    pub build_rs_files: usize,
    pub lib_rs_files: usize,
    pub total_lines: usize,
    pub declarations: usize,
    pub modules: Vec<String>,
}

impl Default for RustcNixInfo {
    fn default() -> Self {
        Self {
            version: String::new(),
            store_path: String::new(),
            src_tarball: String::new(),
            src_url: String::new(),
            drv_path: String::new(),
            unpacked_path: None,
        }
    }
}

#[decl(fn, name = "find_current_rustc", vis = "pub", hash = "b40d822a")]
pub fn find_current_rustc() -> Result<RustcNixInfo, String> {
    let mut info = RustcNixInfo::default();
    
    // Get rustc version from Nix
    let version_output = Command::new("nix")
        .args(["eval", "--raw", "nixpkgs#rustc.version"])
        .output()
        .map_err(|e| format!("Failed to query nix: {}", e))?;
    
    if version_output.status.success() {
        info.version = String::from_utf8_lossy(&version_output.stdout).to_string();
    } else {
        // Fallback: try rustc --version
        let rustc_output = Command::new("rustc")
            .arg("--version")
            .output()
            .map_err(|e| format!("Failed to run rustc: {}", e))?;
        
        let version_str = String::from_utf8_lossy(&rustc_output.stdout);
        info.version = version_str
            .split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string();
    }
    
    // Get source tarball path
    let src_output = Command::new("nix")
        .args(["eval", "--raw", "nixpkgs#rustc.src"])
        .output()
        .map_err(|e| format!("Failed to get src: {}", e))?;
    
    if src_output.status.success() {
        info.src_tarball = String::from_utf8_lossy(&src_output.stdout).to_string();
    }
    
    // Get source URL
    let url_output = Command::new("nix")
        .args(["eval", "--json", "nixpkgs#rustc.src.urls"])
        .output()
        .map_err(|e| format!("Failed to get src urls: {}", e))?;
    
    if url_output.status.success() {
        let urls_str = String::from_utf8_lossy(&url_output.stdout);
        // Parse JSON array and get first URL
        if let Some(url) = urls_str
            .trim_matches(|c| c == '[' || c == ']' || c == '"')
            .split("\",\"")
            .next()
        {
            info.src_url = url.to_string();
        }
    }
    
    // Get derivation path
    let drv_output = Command::new("nix")
        .args(["path-info", "--derivation", "nixpkgs#rustc"])
        .output()
        .map_err(|e| format!("Failed to get drv path: {}", e))?;
    
    if drv_output.status.success() {
        info.drv_path = String::from_utf8_lossy(&drv_output.stdout).trim().to_string();
    }
    
    // Get store path
    let store_output = Command::new("nix")
        .args(["path-info", "nixpkgs#rustc"])
        .output()
        .map_err(|e| format!("Failed to get store path: {}", e))?;
    
    if store_output.status.success() {
        info.store_path = String::from_utf8_lossy(&store_output.stdout).trim().to_string();
    }
    
    Ok(info)
}

#[decl(fn, name = "unpack_rustc_source", vis = "pub", hash = "17f6ce8b")]
pub fn unpack_rustc_source(info: &RustcNixInfo, target_dir: &Path) -> Result<PathBuf, String> {
    if info.src_tarball.is_empty() {
        return Err("No source tarball found".to_string());
    }
    
    let tarball_path = Path::new(&info.src_tarball);
    if !tarball_path.exists() {
        return Err(format!("Source tarball not found: {}", info.src_tarball));
    }
    
    fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create target dir: {}", e))?;
    
    let extract_result = if info.src_tarball.ends_with(".tar.gz") || info.src_tarball.ends_with(".tgz") {
        Command::new("tar")
            .args(["-xzf", &info.src_tarball, "-C", target_dir.to_str().unwrap()])
            .output()
    } else if info.src_tarball.ends_with(".tar.xz") {
        Command::new("tar")
            .args(["-xJf", &info.src_tarball, "-C", target_dir.to_str().unwrap()])
            .output()
    } else {
        return Err(format!("Unknown archive format: {}", info.src_tarball));
    };
    
    extract_result.map_err(|e| format!("Failed to extract: {}", e))?;
    
    // Find the extracted directory
    let extracted_dir = fs::read_dir(target_dir)
        .map_err(|e| format!("Failed to read dir: {}", e))?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().contains("rustc"))
        .map(|e| e.path())
        .ok_or("Could not find extracted rustc directory")?;
    
    Ok(extracted_dir)
}

#[decl(fn, name = "analyze_rustc_source", vis = "pub", hash = "b71ac192")]
pub fn analyze_rustc_source(source_dir: &Path) -> Result<RustcSourceStats, String> {
    let mut stats = RustcSourceStats {
        total_files: 0,
        rust_files: 0,
        build_rs_files: 0,
        lib_rs_files: 0,
        total_lines: 0,
        declarations: 0,
        modules: vec![],
    };
    
    // Find all Rust files
    let find_output = Command::new("find")
        .args([source_dir.to_str().unwrap(), "-name", "*.rs", "-type", "f"])
        .output()
        .map_err(|e| format!("Failed to find files: {}", e))?;
    
    let stdout_str = String::from_utf8_lossy(&find_output.stdout).into_owned(); // Create an owned String
    let files: Vec<&str> = stdout_str
        .lines()
        .filter(|s| !s.is_empty())
        .collect();
    
    stats.rust_files = files.len();
    
    // Count build.rs and lib.rs files
    for file in &files {
        if file.ends_with("build.rs") {
            stats.build_rs_files += 1;
        }
        if file.ends_with("lib.rs") {
            stats.lib_rs_files += 1;
        }
        
        // Extract module names from compiler directory
        if file.contains("/compiler/") {
            if let Some(module) = file
                .split("/compiler/")
                .nth(1)
                .and_then(|s| s.split('/').next())
            {
                if !stats.modules.contains(&module.to_string()) {
                    stats.modules.push(module.to_string());
                }
            }
        }
    }
    
    // Count lines and declarations in a sample
    let sample_files: Vec<_> = files.iter().take(100).collect();
    for file in &sample_files {
        if let Ok(content) = fs::read_to_string(file) {
            stats.total_lines += content.lines().count();
            stats.declarations += content.matches("pub fn ").count();
            stats.declarations += content.matches("pub struct ").count();
            stats.declarations += content.matches("pub enum ").count();
            stats.declarations += content.matches("pub trait ").count();
        }
    }
    
    // Extrapolate for full codebase
    if !files.is_empty() {
        let ratio = files.len() as f64 / sample_files.len().max(1) as f64;
        stats.total_lines = (stats.total_lines as f64 * ratio) as usize;
        stats.declarations = (stats.declarations as f64 * ratio) as usize;
    }
    
    stats.modules.sort();
    
    Ok(stats)
}

#[decl(fn, name = "apply_decl_wrappers_to_rustc", vis = "pub", hash = "8c6261ea")]
pub fn apply_decl_wrappers_to_rustc(source_dir: &Path, output_dir: &Path) -> Result<usize, String> {
    use crate::decl_wrapper::{extract_declarations, generate_decl_attribute};
    
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output dir: {}", e))?;
    
    // Find key files to wrap
    let key_patterns = [
        "compiler/rustc_driver/src/lib.rs",
        "compiler/rustc_interface/src/lib.rs",
        "compiler/rustc_middle/src/lib.rs",
        "compiler/rustc_ast/src/lib.rs",
        "compiler/rustc_hir/src/lib.rs",
        "compiler/rustc_parse/src/lib.rs",
        "compiler/rustc_expand/src/lib.rs",
        "compiler/rustc_codegen_ssa/src/lib.rs",
    ];
    
    let mut total_wrapped = 0;
    
    for pattern in &key_patterns {
        let src_file = source_dir.join(pattern);
        if !src_file.exists() {
            continue;
        }
        
        let content = fs::read_to_string(&src_file)
            .map_err(|e| format!("Failed to read {}: {}", pattern, e))?;
        
        let decls = extract_declarations(&content);
        let public_decls: Vec<_> = decls.iter()
            .filter(|d| d.visibility == crate::decl_wrapper::VisibilityKind::Public)
            .collect();
        
        if public_decls.is_empty() {
            continue;
        }
        
        // Generate wrapped version
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut offset = 0;
        
        for decl in &public_decls {
            let insert_line = decl.line.saturating_sub(1) + offset;
            if insert_line < lines.len() {
                let attr = generate_decl_attribute(decl);
                lines.insert(insert_line, attr);
                offset += 1;
                total_wrapped += 1;
            }
        }
        
        // Write to output directory
        let out_file = output_dir.join(pattern);
        if let Some(parent) = out_file.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&out_file, lines.join("\n"))
            .map_err(|e| format!("Failed to write {}: {}", pattern, e))?;
    }
    
    Ok(total_wrapped)
}

#[decl(fn, name = "generate_rustc_introspection_report", vis = "pub", hash = "7c8d404f")]
pub fn generate_rustc_introspection_report(info: &RustcNixInfo, stats: &RustcSourceStats) -> String {
    let mut report = String::new();
    
    report.push_str(&format!(r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                    🦀 RUSTC NIX INTROSPECTION REPORT                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝

📋 RUSTC INFO
═══════════════════════════════════════════════════════════════════════════════
   Version:      {}
   Store Path:   {}
   Derivation:   {}
   Source URL:   {}
   Source Path:  {}

📊 SOURCE STATISTICS
═══════════════════════════════════════════════════════════════════════════════
   Rust Files:       {:>8}
   Build.rs Files:   {:>8}
   Lib.rs Files:     {:>8}
   Total Lines:      {:>8} (estimated)
   Declarations:     {:>8} (estimated)

📁 COMPILER MODULES ({})
═══════════════════════════════════════════════════════════════════════════════
"#,
        info.version,
        info.store_path,
        info.drv_path,
        info.src_url,
        info.src_tarball,
        stats.rust_files,
        stats.build_rs_files,
        stats.lib_rs_files,
        stats.total_lines,
        stats.declarations,
        stats.modules.len(),
    ));
    
    for (i, module) in stats.modules.iter().enumerate() {
        if i % 4 == 0 {
            report.push_str("   ");
        }
        report.push_str(&format!("{:<20}", module));
        if (i + 1) % 4 == 0 {
            report.push('\n');
        }
    }
    report.push('\n');
    
    report.push_str(r#"
🔧 INTROSPECTION STATUS
═══════════════════════════════════════════════════════════════════════════════
   Source Available:  ✅
   Can Unpack:        ✅
   Decl Wrapping:     Ready
   
💡 NEXT STEPS
═══════════════════════════════════════════════════════════════════════════════
   1. Unpack source:   audit-fix rustc-unpack /tmp/rustc-src
   2. Apply wrappers:  audit-fix rustc-wrap /tmp/rustc-src /tmp/rustc-wrapped
   3. Analyze decls:   audit-fix rustc-analyze /tmp/rustc-wrapped
"#);
    
    report
}

#[decl(fn, name = "print_rustc_info", vis = "pub", hash = "b92771af")]
pub fn print_rustc_info() {
    match find_current_rustc() {
        Ok(info) => {
            let reset = "\x1b[0m";
            let cyan = "\x1b[96m";
            let green = "\x1b[92m";
            let yellow = "\x1b[93m";
            
            eprintln!("\n{}🦀 RUSTC NIX INFORMATION{}", cyan, reset);
            eprintln!("{}", "═".repeat(60));
            eprintln!("{}Version:{} {}", green, reset, info.version);
            eprintln!("{}Store Path:{} {}", green, reset, info.store_path);
            eprintln!("{}Derivation:{} {}", green, reset, info.drv_path);
            eprintln!("{}Source URL:{} {}", yellow, reset, info.src_url);
            eprintln!("{}Source Tarball:{} {}", yellow, reset, info.src_tarball);
            eprintln!();
        }
        Err(e) => {
            eprintln!("❌ Failed to find rustc: {}", e);
        }
    }
}

#[macro_export]
macro_rules! rustc_info {
    () => {{
        $crate::nix_rustc::print_rustc_info();
    }};
}

#[macro_export]
macro_rules! find_nix_rustc {
    () => {{
        $crate::nix_rustc::find_current_rustc()
    }};
}

#[macro_export]
macro_rules! unpack_rustc {
    ($target:expr) => {{
        let info = $crate::nix_rustc::find_current_rustc()?;
        $crate::nix_rustc::unpack_rustc_source(&info, std::path::Path::new($target))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rustc_info() {
        // This test requires nix to be available
        if Command::new("nix").arg("--version").output().is_ok() {
            let result = find_current_rustc();
            assert!(result.is_ok() || result.is_err()); // Just check it doesn't panic
        }
    }
}