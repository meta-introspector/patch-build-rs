# Examples for `mkslop!` and `#[lru]`

This document provides canonical examples for the `mkslop!` procedural macro and the `#[lru]` attribute macro. These examples are referenced from other parts of the documentation.

## `mkslop!`

The `mkslop!` procedural macro is designed to automatically fix common issues in generated code, with a focus on malformed format strings that can be produced by AI code generators. It is used internally by `mkbuildrs!` but can also be used as a standalone tool.

**Example:**

```rust
use mkslop_macros::mkslop;

fn main() {
    // mkslop! will fix the potentially problematic format string
    println!("{}", mkslop!("cargo:rustc-cfg={0}=\"{1}\"", "feature", "enabled"));
}
```

## `#[lru]`

The `#[lru]` attribute macro adds memoization (caching) to a function. This is particularly useful in `build.rs` scripts for caching the results of expensive operations, like querying a database or reading a file, making your builds faster and more deterministic.

**Example:**

```rust
use patch_build_rs_macros::lru;
use std::fs;
use std::path::PathBuf;

#[lru(capacity = 10)]
fn read_config_file(path: PathBuf) -> String {
    // This file will only be read from disk once per path
    fs::read_to_string(path).unwrap()
}
```
