# Rust-Nix Integration System

This system allows `build.rs` scripts to drive Rust compiler patching and rebuilding through Nix.

## New Macros

### `nix_rust_src!()`
Finds Rust compiler source code in the Nix store.

```rust
let rust_path = nix_rust_src!();
// Returns: "/nix/store/...-rustc-1.75.0-src"
```

### `extract_decl!("path/to/file.rs")`
Extracts function declarations from Rust source files.

```rust
let decls = extract_decl!("compiler/rustc/src/main.rs");
// Returns: "pub fn main() { ... }\nfn helper() { ... }"
```

### `patch_rust!("description")`
Creates patch environment and prepares Rust rebuild.

```rust
let result = patch_rust!("Add macro introspection");
// Creates extraction environment and Nix build setup
```

## Usage in build.rs

```rust
use patch_build_rs_macros::{mkbuildrs, nix_rust_src, extract_decl, patch_rust};

fn main() {
    mkbuildrs! {
        description: "Rust compiler integration";
        nix_integration: true;
    }
    
    let rust_src = nix_rust_src!();
    let decls = extract_decl!("compiler/rustc_driver/src/lib.rs");
    let patched = patch_rust!("Custom compiler hooks");
    
    println!("cargo:rustc-env=CUSTOM_RUST={}", patched);
}
```

## Nix Integration

Use `nix/patched-rust.nix` to build the patched compiler:

```bash
nix-build nix/patched-rust.nix
```

This creates a Rust compiler with macro introspection hooks added by the automorphic system.

## Files Created

- `patch-build-rs-macros/src/rust_nix.rs` - Core Rust-Nix integration
- `examples/rust_nix_build.rs` - Example build.rs usage  
- `nix/patched-rust.nix` - Nix expression for patched Rust
- This documentation file

The system is modular and can be included in any project's build.rs without overwriting existing files.
