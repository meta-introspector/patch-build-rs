use patch_build_rs_macros::mkbuildrs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Simple mkbuildrs usage - just generate a message for now
    let _build_system = mkbuildrs!("simple");
    
    println!("cargo:rustc-env=SYSTEM_VERSION=1.0.0");
    println!("cargo:warning=🦀 Automorphic System build complete");
}
