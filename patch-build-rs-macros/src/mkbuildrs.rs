use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

pub fn mkbuildrs_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔧 mkbuildrs: {}", #config);
            
            let build_rs_code = format!(r###"
// Auto-generated build.rs with Nix integration
// Config: {}

use std::process::Command;
use std::path::Path;
use std::env;

fn main() {{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Nix integration for Rust versions
    setup_nix_rust_versions();
    
    // Download and cache Rust versions
    cache_rust_versions();
    
    // Setup build environment
    setup_build_environment();
    
    println!("cargo:warning=🔧 mkbuildrs build complete");
}}

fn setup_nix_rust_versions() {{
    println!("🔧 Setting up Nix Rust versions...");
    
    // Check if nix is available
    let nix_available = Command::new("which")
        .arg("nix")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    if nix_available {{
        // Create nix expressions for different Rust versions
        let rust_versions = ["1.70.0", "1.75.0", "1.80.0", "stable", "beta", "nightly"];
        
        for version in &rust_versions {{
            let nix_expr = format!(r#"
# Rust {{}} via Nix
{{ pkgs ? import <nixpkgs> {{}} }}:
pkgs.rustc.override {{
  version = "{}";
}}
            "#, version, version);
            
            std::fs::write(format!("nix/rust-{{}}.nix", version), nix_expr).ok();
        }}
        
        println!("cargo:warning=✅ Nix Rust expressions created");
    }} else {{
        println!("cargo:warning=⚠️ Nix not available, using rustup fallback");
    }}
}}

fn cache_rust_versions() {{
    println!("📦 Caching Rust versions...");
    
    let cache_dir = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string()) + "/rust-cache";
    
    std::fs::create_dir_all(&cache_dir).ok();
    
    // Cache different Rust versions
    let versions = ["1.70.0", "1.75.0", "1.80.0"];
    
    for version in &versions {{
        let version_dir = format!("{{}}/rust-{{}}", cache_dir, version);
        
        if !Path::new(&version_dir).exists() {{
            println!("📥 Downloading Rust {{}}...", version);
            
            // Try Nix first, then rustup
            let success = download_via_nix(version, &version_dir) ||
                         download_via_rustup(version, &version_dir) ||
                         download_via_github(version, &version_dir);
            
            if success {{
                println!("cargo:warning=✅ Rust {{}} cached", version);
            }} else {{
                println!("cargo:warning=❌ Failed to cache Rust {{}}", version);
            }}
        }}
    }}
}}

fn download_via_nix(version: &str, target_dir: &str) -> bool {{
    let nix_expr = format!("nix/rust-{{}}.nix", version);
    
    if Path::new(&nix_expr).exists() {{
        let output = Command::new("nix-build")
            .arg(&nix_expr)
            .arg("-o")
            .arg(target_dir)
            .output();
            
        output.map(|o| o.status.success()).unwrap_or(false)
    }} else {{
        false
    }}
}}

fn download_via_rustup(version: &str, target_dir: &str) -> bool {{
    // Install specific version via rustup
    let install = Command::new("rustup")
        .args(&["toolchain", "install", version])
        .output();
        
    if install.map(|o| o.status.success()).unwrap_or(false) {{
        // Copy to cache directory
        let rustup_path = format!("~/.rustup/toolchains/{{}}", version);
        let copy = Command::new("cp")
            .args(&["-r", &rustup_path, target_dir])
            .output();
            
        copy.map(|o| o.status.success()).unwrap_or(false)
    }} else {{
        false
    }}
}}

fn download_via_github(version: &str, target_dir: &str) -> bool {{
    // Download from GitHub releases
    let url = format!("https://github.com/rust-lang/rust/archive/{{}}.tar.gz", version);
    
    let download = Command::new("curl")
        .args(&["-L", &url, "-o", "/tmp/rust.tar.gz"])
        .output();
        
    if download.map(|o| o.status.success()).unwrap_or(false) {{
        let extract = Command::new("tar")
            .args(&["xzf", "/tmp/rust.tar.gz", "-C", target_dir, "--strip-components=1"])
            .output();
            
        extract.map(|o| o.status.success()).unwrap_or(false)
    }} else {{
        false
    }}
}}

fn setup_build_environment() {{
    println!("⚙️ Setting up build environment...");
    
    // Set environment variables for cached Rust versions
    let cache_dir = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string()) + "/rust-cache";
    
    println!("cargo:rustc-env=RUST_CACHE_DIR={{}}", cache_dir);
    
    // Generate build macros for each cached version
    generate_version_macros(&cache_dir);
}}

fn generate_version_macros(cache_dir: &str) {{
    let macro_file = format!("{{}}/version_macros.rs", cache_dir);
    
    let macro_content = r#"
// Auto-generated version macros

macro_rules! rust_version {{
    ("1.70.0") => {{ include_str!(concat!(env!("RUST_CACHE_DIR"), "/rust-1.70.0/VERSION")) }};
    ("1.75.0") => {{ include_str!(concat!(env!("RUST_CACHE_DIR"), "/rust-1.75.0/VERSION")) }};
    ("1.80.0") => {{ include_str!(concat!(env!("RUST_CACHE_DIR"), "/rust-1.80.0/VERSION")) }};
    ("stable") => {{ include_str!(concat!(env!("RUST_CACHE_DIR"), "/rust-stable/VERSION")) }};
}}

macro_rules! rust_source {{
    ($version:expr) => {{
        concat!(env!("RUST_CACHE_DIR"), "/rust-", $version, "/src")
    }};
}}

macro_rules! rust_binary {{
    ($version:expr) => {{
        concat!(env!("RUST_CACHE_DIR"), "/rust-", $version, "/bin/rustc")
    }};
}}
    "#;
    
    std::fs::write(macro_file, macro_content).ok();
    println!("cargo:warning=🔧 Version macros generated");
}}
            "###, #config);
            
            build_rs_code
        }
    }.into()
}

pub fn nix_rust_version_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let version = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🦀 Nix Rust version: {}", #version);
            
            // Generate Nix expression for specific Rust version
            let nix_expr = format!(r###"
# Nix expression for Rust {}
{{ pkgs ? import <nixpkgs> {{}} }}:

let
  rustVersion = "{}";
  rustSrc = pkgs.fetchFromGitHub {{
    owner = "rust-lang";
    repo = "rust";
    rev = rustVersion;
    sha256 = "0000000000000000000000000000000000000000000000000000";
  }};
in
pkgs.rustc.override {{
  version = rustVersion;
  src = rustSrc;
}}
            "###, #version, #version);
            
            nix_expr
        }
    }.into()
}

pub fn rust_cache_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let cache_config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=📦 Rust cache: {}", #cache_config);
            
            let cache_system = format!(r###"
// Rust Version Cache System
// Config: {}

use std::path::{{Path, PathBuf}};
use std::collections::HashMap;

pub struct RustCache {{
    cache_dir: PathBuf,
    versions: HashMap<String, PathBuf>,
}}

impl RustCache {{
    pub fn new() -> Self {{
        let cache_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| "target".to_string())
            .into();
        let cache_dir = cache_dir.join("rust-cache");
        
        Self {{
            cache_dir,
            versions: HashMap::new(),
        }}
    }}
    
    pub fn get_version(&self, version: &str) -> Option<&PathBuf> {{
        self.versions.get(version)
    }}
    
    pub fn cache_version(&mut self, version: &str) -> Result<PathBuf, String> {{
        let version_dir = self.cache_dir.join(format!("rust-{{}}", version));
        
        if version_dir.exists() {{
            self.versions.insert(version.to_string(), version_dir.clone());
            return Ok(version_dir);
        }}
        
        // Download and cache the version
        self.download_version(version, &version_dir)?;
        self.versions.insert(version.to_string(), version_dir.clone());
        
        Ok(version_dir)
    }}
    
    fn download_version(&self, version: &str, target: &Path) -> Result<(), String> {{
        std::fs::create_dir_all(target)
            .map_err(|e| format!("Failed to create cache dir: {{}}", e))?;
        
        // Try multiple download methods
        if self.download_via_nix(version, target).is_ok() {{
            return Ok(());
        }}
        
        if self.download_via_rustup(version, target).is_ok() {{
            return Ok(());
        }}
        
        if self.download_via_github(version, target).is_ok() {{
            return Ok(());
        }}
        
        Err(format!("Failed to download Rust version {{}}", version))
    }}
    
    fn download_via_nix(&self, version: &str, target: &Path) -> Result<(), String> {{
        use std::process::Command;
        
        let nix_expr = format!(r#"
        {{ pkgs ? import <nixpkgs> {{}} }}:
        pkgs.rustc.override {{ version = "{}"; }}
        "#, version);
        
        let temp_file = "/tmp/rust-version.nix";
        std::fs::write(temp_file, nix_expr)
            .map_err(|e| format!("Failed to write nix expr: {{}}", e))?;
        
        let output = Command::new("nix-build")
            .arg(temp_file)
            .arg("-o")
            .arg(target)
            .output()
            .map_err(|e| format!("Nix build failed: {{}}", e))?;
        
        if output.status.success() {{
            Ok(())
        }} else {{
            Err("Nix build failed".to_string())
        }}
    }}
    
    fn download_via_rustup(&self, version: &str, target: &Path) -> Result<(), String> {{
        use std::process::Command;
        
        // Install via rustup
        let install = Command::new("rustup")
            .args(&["toolchain", "install", version])
            .output()
            .map_err(|e| format!("Rustup install failed: {{}}", e))?;
        
        if !install.status.success() {{
            return Err("Rustup install failed".to_string());
        }}
        
        // Copy to cache
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let rustup_path = format!("{{}}/.rustup/toolchains/{{}}", home, version);
        
        let copy = Command::new("cp")
            .args(&["-r", &rustup_path, &target.to_string_lossy()])
            .output()
            .map_err(|e| format!("Copy failed: {{}}", e))?;
        
        if copy.status.success() {{
            Ok(())
        }} else {{
            Err("Copy failed".to_string())
        }}
    }}
    
    fn download_via_github(&self, version: &str, target: &Path) -> Result<(), String> {{
        use std::process::Command;
        
        let url = format!("https://github.com/rust-lang/rust/archive/{{}}.tar.gz", version);
        let temp_file = "/tmp/rust-source.tar.gz";
        
        // Download
        let download = Command::new("curl")
            .args(&["-L", &url, "-o", temp_file])
            .output()
            .map_err(|e| format!("Download failed: {{}}", e))?;
        
        if !download.status.success() {{
            return Err("Download failed".to_string());
        }}
        
        // Extract
        let extract = Command::new("tar")
            .args(&["xzf", temp_file, "-C", &target.to_string_lossy(), "--strip-components=1"])
            .output()
            .map_err(|e| format!("Extract failed: {{}}", e))?;
        
        if extract.status.success() {{
            Ok(())
        }} else {{
            Err("Extract failed".to_string())
        }}
    }}
}}

// Convenience macros for cached versions
macro_rules! cached_rust {{
    ($version:expr) => {{
        RustCache::new().cache_version($version)
    }};
}}

macro_rules! rust_src_path {{
    ($version:expr) => {{
        cached_rust!($version).map(|p| p.join("src"))
    }};
}}

macro_rules! rust_bin_path {{
    ($version:expr) => {{
        cached_rust!($version).map(|p| p.join("bin/rustc"))
    }};
}}
            "###, #cache_config);
            
            cache_system
        }
    }.into()
}
