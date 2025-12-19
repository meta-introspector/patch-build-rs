// build.rs that generates the Automorphic Ring of Rust
// Include this to analyze rustc structure at build time

use patch_build_rs_macros::{
    mkbuildrs, analyze_rustc_ring, dependency_graph, ring_properties
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    mkbuildrs! {
        description: "Automorphic Ring of Rust analysis";
        generates: "rustc dependency graph and mathematical properties";
        output: ["rustc_ring.dot", "ring_properties.txt"];
    }
    
    // Generate the complete ring analysis
    let ring_data = analyze_rustc_ring!();
    let graph = dependency_graph!();
    let properties = ring_properties!();
    
    // Export as environment variables
    println!("cargo:rustc-env=RUSTC_RING_DATA={}", ring_data);
    println!("cargo:rustc-env=RUSTC_GRAPH_SIZE={}", graph.len());
    
    // Save analysis files
    std::fs::write("target/rustc_ring.dot", graph).ok();
    std::fs::write("target/ring_properties.txt", properties).ok();
    
    println!("cargo:warning=🔮 Automorphic Ring of Rust analysis complete");
}
