// mkbuildrs Demo - Nix-integrated build system with Rust version caching
// Generates build.rs that manages multiple Rust versions via Nix

use patch_build_rs_macros::{
    mkbuildrs, nix_rust_version, rust_cache
};

fn main() {
    println!("🔧 mkbuildrs Demo - Nix-Integrated Build System");
    
    // Generate complete build.rs with Nix integration
    let config = "nix_integration=true,cache_versions=true,auto_download=true";
    let build_rs = mkbuildrs!(config);
    
    // Generate Nix expressions for specific Rust versions
    let rust_1_70 = nix_rust_version!("1.70.0");
    let rust_1_75 = nix_rust_version!("1.75.0");
    let rust_stable = nix_rust_version!("stable");
    let rust_nightly = nix_rust_version!("nightly");
    
    // Generate Rust version cache system
    let cache_config = "versions=[1.70.0,1.75.0,1.80.0,stable],methods=[nix,rustup,github]";
    let cache_system = rust_cache!(cache_config);
    
    println!("🔧 Build system: {} lines", build_rs.lines().count());
    println!("🦀 Nix expressions: 4 versions");
    println!("📦 Cache system: {} lines", cache_system.lines().count());
    
    // Save all generated components
    std::fs::create_dir_all("nix").ok();
    std::fs::create_dir_all("src").ok();
    
    // Save the generated build.rs
    std::fs::write("build.rs", &build_rs).ok();
    
    // Save Nix expressions
    std::fs::write("nix/rust-1.70.0.nix", &rust_1_70).ok();
    std::fs::write("nix/rust-1.75.0.nix", &rust_1_75).ok();
    std::fs::write("nix/rust-stable.nix", &rust_stable).ok();
    std::fs::write("nix/rust-nightly.nix", &rust_nightly).ok();
    
    // Save cache system
    std::fs::write("src/rust_cache.rs", &cache_system).ok();
    
    // Create shell.nix for development
    std::fs::write("shell.nix", r#"
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Multiple Rust versions
    (import ./nix/rust-1.70.0.nix { inherit pkgs; })
    (import ./nix/rust-1.75.0.nix { inherit pkgs; })
    (import ./nix/rust-stable.nix { inherit pkgs; })
    
    # Build tools
    cargo
    rustfmt
    clippy
    
    # System tools
    curl
    tar
    git
  ];
  
  shellHook = ''
    echo "🔧 mkbuildrs development environment"
    echo "🦀 Multiple Rust versions available via Nix"
    echo "📦 Run 'cargo build' to test version caching"
  '';
}
    "#).ok();
    
    // Create usage example
    std::fs::write("src/main.rs", r#"
// Example usage of cached Rust versions
mod rust_cache;

use rust_cache::RustCache;

fn main() {
    println!("🔧 mkbuildrs - Rust Version Manager");
    
    let mut cache = RustCache::new();
    
    // Cache different Rust versions
    let versions = ["1.70.0", "1.75.0", "stable"];
    
    for version in &versions {
        match cache.cache_version(version) {
            Ok(path) => println!("✅ Rust {} cached at: {:?}", version, path),
            Err(e) => println!("❌ Failed to cache Rust {}: {}", version, e),
        }
    }
    
    // Use cached versions
    if let Some(stable_path) = cache.get_version("stable") {
        println!("🦀 Stable Rust available at: {:?}", stable_path);
    }
}
    "#).ok();
    
    // Create comprehensive documentation
    std::fs::write("MKBUILDRS.md", format!(
        r#"# 🔧 mkbuildrs - Nix-Integrated Rust Build System

## Overview

The `mkbuildrs!` macro generates a complete build.rs that:
- Integrates with Nix for reproducible Rust versions
- Downloads and caches multiple Rust versions
- Provides build macros for version management
- Supports fallback methods (rustup, GitHub)

## Generated Components

### build.rs
{}

### Nix Expressions
- `nix/rust-1.70.0.nix` - Rust 1.70.0 via Nix
- `nix/rust-1.75.0.nix` - Rust 1.75.0 via Nix  
- `nix/rust-stable.nix` - Stable Rust via Nix
- `nix/rust-nightly.nix` - Nightly Rust via Nix

### Cache System
{}

## Usage

### Basic Setup
```bash
# Enter Nix shell with multiple Rust versions
nix-shell

# Build with version caching
cargo build
```

### Programmatic Usage
```rust
use rust_cache::RustCache;

let mut cache = RustCache::new();
let rust_path = cache.cache_version("1.75.0")?;
```

### Build Macros
```rust
// Generated automatically in build.rs
rust_version!("1.75.0")  // Get version string
rust_source!("1.75.0")   // Get source path
rust_binary!("1.75.0")   // Get binary path
```

## Features

### Multi-Method Download
1. **Nix** (preferred) - Reproducible, cached
2. **Rustup** (fallback) - Standard Rust installer
3. **GitHub** (last resort) - Direct source download

### Version Management
- Automatic caching in `target/rust-cache/`
- Version detection and reuse
- Parallel downloads for multiple versions

### Build Integration
- Cargo integration via build.rs
- Environment variable setup
- Macro generation for version access

## Configuration

The `mkbuildrs!` macro accepts configuration:
```rust
mkbuildrs!("nix_integration=true,cache_versions=true,auto_download=true")
```

Options:
- `nix_integration` - Enable Nix expressions
- `cache_versions` - Cache downloaded versions
- `auto_download` - Download missing versions automatically

## Requirements

### For Nix Integration
- Nix package manager installed
- `<nixpkgs>` channel available

### For Rustup Fallback
- Rustup installed and configured

### For GitHub Fallback
- `curl` and `tar` available

## 🎯 Benefits

- **Reproducible builds** via Nix
- **Multiple Rust versions** in same project
- **Automatic caching** reduces download time
- **Fallback methods** ensure availability
- **Build macro integration** for easy access

**🔧 Complete Rust version management through build macros!**
        "#,
        build_rs.lines().take(50).collect::<Vec<_>>().join("\n"),
        cache_system.lines().take(30).collect::<Vec<_>>().join("\n")
    )).ok();
    
    println!("💾 mkbuildrs system generated!");
    println!("🔧 build.rs: Complete Nix-integrated build system");
    println!("🦀 Nix expressions: 4 Rust versions available");
    println!("📦 Cache system: Multi-method download and caching");
    println!("🐚 shell.nix: Development environment ready");
    println!("📋 MKBUILDRS.md: Complete documentation");
    println!("");
    println!("🚀 Usage:");
    println!("  nix-shell          # Enter development environment");
    println!("  cargo build        # Test version caching");
    println!("  cargo run          # Demo version management");
}
