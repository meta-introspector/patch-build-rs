use patch_build_rs_macros::mkbuildrs_checked;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Generate enhanced build system with syntax-checked macros
    let _build_system = mkbuildrs_checked!("syntax_checked");
    
    println!("cargo:rustc-env=SYSTEM_VERSION=1.0.0");
    println!("cargo:warning=🦀 Automorphic System build complete");
}
