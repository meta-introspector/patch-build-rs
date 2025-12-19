use patch_build_rs_macros::mkbuildrs_with_macros;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Generate enhanced build system with common macros
    let _build_system = mkbuildrs_with_macros!("enhanced");
    
    println!("cargo:rustc-env=SYSTEM_VERSION=1.0.0");
    println!("cargo:warning=🦀 Automorphic System build complete");
}
