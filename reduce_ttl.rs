use patch_build_rs_macros::{simplify, pii, prune, compress, pipeline};
use std::fs;

fn main() {
    println!("🔍 Reading massive TTL file...");
    
    // Read the massive file (first 10MB only to avoid memory issues)
    let content = fs::read_to_string("output/grandparent_recursive.ttl")
        .unwrap_or_else(|_| "No TTL file found".to_string());
    
    let sample = content.chars().take(10_000_000).collect::<String>(); // 10MB sample
    
    println!("📊 Original sample: {} chars", sample.len());
    
    // Apply reduction pipeline
    let reduced = pipeline!(&sample);
    
    // Save reduced version
    fs::write("output/reduced_grandparent.ttl", &reduced).ok();
    
    println!("✅ Reduction complete!");
    println!("📉 Size reduction: {} -> {} chars ({:.1}% reduction)", 
        sample.len(), 
        reduced.len(),
        (1.0 - reduced.len() as f64 / sample.len() as f64) * 100.0
    );
}
