// Example build.rs that drives Rust compiler patching
// Include this in your project's build.rs

use patch_build_rs_macros::{mkbuildrs, nix_rust_src, extract_decl, patch_rust};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    mkbuildrs! {
        description: "Build system with Rust compiler integration";
        nix_integration: true;
    }
    
    // Find Rust source in Nix store
    let rust_src_path = nix_rust_src!();
    
    // Extract declarations from key Rust files
    let main_decls = extract_decl!("compiler/rustc/src/main.rs");
    let driver_decls = extract_decl!("compiler/rustc_driver/src/lib.rs");
    
    // Apply patches (creates extraction environments)
    let patch_result = patch_rust!("Add macro introspection hooks");
    
    println!("cargo:rustc-env=RUST_SRC_PATH={}", rust_src_path);
    println!("cargo:rustc-env=PATCH_STATUS={}", patch_result);
}
