use nix_rust_macros::{RustInNix, cargo_source, rust_as_macro};

fn main() {
    // Find and inject macros into stable Rust in Nix store
    RustInNix!("stable");
    
    // Or work with a specific version
    RustInNix!("1.70.0");
    
    // Clone and inject macros into Cargo source
    cargo_source!("https://github.com/rust-lang/cargo.git");
    
    // Convert Rust compiler into a macro
    rust_as_macro!();
    
    // Now you can use Rust as a macro
    let result = rust_compiler!(r#"
        fn main() {
            println!("Hello from macro-compiled Rust!");
        }
    "#);
    
    println!("Compilation result: {}", result);
}
