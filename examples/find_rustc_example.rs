// examples/find_rustc_example.rs
// This file demonstrates the usage of the find_nix_rustc! macro.

use nix2proc_macros::find_nix_rustc;

fn main() {
    println!("Searching for Rust compilers in /nix/store...");

    // Example 1: Find all rustc paths
    let all_rustc_paths: Vec<String> = find_nix_rustc!();
    println!("\nFound ALL Rust compilers:");
    for path in all_rustc_paths {
        println!("  {}\n", path);
    }

    // Example 2: Find rustc paths containing "1.91"
    let rustc_191_paths: Vec<String> = find_nix_rustc!("1.91");
    println!("\nFound Rust compilers containing '1.91':");
    for path in rustc_191_paths {
        println!("  {}\n", path);
    }
    
    println!("\nSearch complete.");
}