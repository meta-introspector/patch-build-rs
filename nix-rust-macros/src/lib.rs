use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::process::Command;
use std::fs;
use std::path::Path;

#[proc_macro]
pub fn RustInNix(input: TokenStream) -> TokenStream {
    let version = if input.is_empty() {
        "stable".to_string()
    } else {
        parse_macro_input!(input as LitStr).value()
    };
    
    let rust_info = find_rust_in_nix(&version).unwrap_or_else(|_| {
        panic!("Failed to find Rust {} in Nix store", version)
    });
    
    let output = quote! {
        {
            const RUST_NIX_PATH: &str = #rust_info.path;
            const RUST_VERSION: &str = #rust_info.version;
            const RUST_SOURCE: &str = #rust_info.source_path;
            
            // Inject our macros into the build
            inject_macros_into_rust_build(RUST_SOURCE);
        }
    };
    
    output.into()
}

struct RustInfo {
    path: String,
    version: String,
    source_path: String,
}

fn find_rust_in_nix(version: &str) -> Result<RustInfo, Box<dyn std::error::Error>> {
    // Find Rust in Nix store
    let output = Command::new("nix-store")
        .args(&["-q", "--references", "/run/current-system"])
        .output()?;
    
    let store_paths = String::from_utf8_lossy(&output.stdout);
    let rust_path = store_paths
        .lines()
        .find(|line| line.contains("rust") && line.contains(version))
        .ok_or("Rust not found in Nix store")?;
    
    // Get Rust source
    let source_path = download_rust_source(version)?;
    
    Ok(RustInfo {
        path: rust_path.to_string(),
        version: version.to_string(),
        source_path,
    })
}

fn download_rust_source(version: &str) -> Result<String, Box<dyn std::error::Error>> {
    let temp_dir = "/tmp/rust-source";
    fs::create_dir_all(temp_dir)?;
    
    // Download and extract Rust source
    let url = format!("https://github.com/rust-lang/rust/archive/{}.tar.gz", version);
    Command::new("curl")
        .args(&["-L", &url, "-o", &format!("{}/rust.tar.gz", temp_dir)])
        .output()?;
    
    Command::new("tar")
        .args(&["-xzf", &format!("{}/rust.tar.gz", temp_dir), "-C", temp_dir])
        .output()?;
    
    Ok(format!("{}/rust-{}", temp_dir, version))
}

fn inject_macros_into_rust_build(source_path: &str) {
    // Replace build.rs files with our macro-wrapped versions
    inject_build_rs_macros(source_path);
    
    // Replace bootstrap.py with our macro declarations
    inject_bootstrap_macros(source_path);
}

fn inject_build_rs_macros(source_path: &str) {
    let build_rs_files = find_build_rs_files(source_path);
    
    for build_rs in build_rs_files {
        let original = fs::read_to_string(&build_rs).unwrap_or_default();
        let wrapped = format!(r#"
use patch_build_rs_macros::{{mkbuildrs, autowrap}};

mkbuildrs! {{
    original_build: r#"{}"#;
    inject_macros: true;
    wrap_functions: true;
}}

fn main() {{
    autowrap!(r#"{}"#);
}}
"#, original.replace('"', r#"\""#), original.replace('"', r#"\""#));
        
        fs::write(&build_rs, wrapped).unwrap();
    }
}

fn inject_bootstrap_macros(source_path: &str) {
    let bootstrap_path = format!("{}/src/bootstrap/bootstrap.py", source_path);
    if Path::new(&bootstrap_path).exists() {
        let bootstrap_wrapper = r#"
#!/usr/bin/env python3
# Macro-wrapped bootstrap

import sys
import os

# Declare build stages via macro
STAGES = {
    "stage0": ["build", "std"],
    "stage1": ["build", "rustc", "std"],  
    "stage2": ["build", "rustc", "std", "tools"]
}

def main():
    stage = sys.argv[1] if len(sys.argv) > 1 else "stage2"
    print(f"Building Rust with macro-injected stage: {stage}")
    
    # Execute original bootstrap logic here
    # This would call the original bootstrap.py functionality
    
if __name__ == "__main__":
    main()
"#;
        fs::write(&bootstrap_path, bootstrap_wrapper).unwrap();
    }
}

fn find_build_rs_files(source_path: &str) -> Vec<String> {
    let output = Command::new("find")
        .args(&[source_path, "-name", "build.rs", "-type", "f"])
        .output()
        .unwrap();
    
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[proc_macro]
pub fn cargo_source(input: TokenStream) -> TokenStream {
    let repo_url = parse_macro_input!(input as LitStr).value();
    
    let output = quote! {
        {
            let temp_dir = "/tmp/cargo-source";
            std::fs::create_dir_all(temp_dir).unwrap();
            
            std::process::Command::new("git")
                .args(&["clone", #repo_url, temp_dir])
                .output()
                .expect("Failed to clone cargo source");
                
            inject_macros_into_rust_build(temp_dir);
        }
    };
    
    output.into()
}

#[proc_macro]
pub fn rust_as_macro(_input: TokenStream) -> TokenStream {
    let output = quote! {
        macro_rules! rust_compiler {
            ($code:expr) => {
                {
                    use std::process::Command;
                    let temp_file = "/tmp/rust_macro_compile.rs";
                    std::fs::write(temp_file, $code).unwrap();
                    
                    let output = Command::new("rustc")
                        .args(&[temp_file])
                        .output()
                        .expect("Failed to compile with Rust macro");
                        
                    String::from_utf8_lossy(&output.stdout)
                }
            };
        }
    };
    
    output.into()
}
