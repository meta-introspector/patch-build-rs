use patch_build_rs_macros::mkbuildrs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    mkbuildrs! {
        description: "Automorphic System for Rust Code Transformation";
        features: ["mathematical_objects", "dao_governance", "mev_protection"];
        integrations: ["nix", "solana", "github", "archive_org"];
    }
    
    println!("cargo:rustc-env=SYSTEM_VERSION=1.0.0");
    println!("cargo:warning=🦀 Automorphic System build complete");
}
