#!/usr/bin/env bash
set -e

echo "Building Rust with injected macros..."

# Build the macro-injected Rust compiler
nix-build rust-with-macros.nix -o result-rust-macros

# Test the macro-injected compiler
echo "Testing macro-injected Rust compiler..."
./result-rust-macros/bin/rustc --version

# Build a test program with the macro-injected compiler
echo 'fn main() { println!("Built with macro-injected Rust!"); }' > test.rs
./result-rust-macros/bin/rustc test.rs -o test
./test

echo "Macro-injected Rust build complete!"
