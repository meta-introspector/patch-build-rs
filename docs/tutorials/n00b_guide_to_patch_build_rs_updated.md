# N00b's Guide to `patch-build-rs`

Welcome to `patch-build-rs`! This project provides powerful tools for generating and modifying Rust `build.rs` scripts, especially useful in complex build environments or when dealing with AI-generated code.

## What is `patch-build-rs`?

At its core, `patch-build-rs` is a **metaprogramming system** for Rust. Think of it as a smart helper that can:

1.  **Generate `build.rs` Scripts Declaratively**: Instead of writing verbose `build.rs` files by hand, you can describe what you need in a concise way using the `mkbuildrs!` macro.
2.  **Apply Semantic Patches**: It automatically fixes common issues in generated code, particularly problems with format strings that can arise from AI-generated Rust code. This is done via the `mkslop!` macro.
3.  **Incorporate Environmental Data**: It can extract information about your build environment (like resource requirements, secrets, systemd services) and embed it into the build process, making it easier for external systems (like Nix) to understand and manage your project.

In essence, `patch-build-rs` helps your Rust project "read itself in and write itself out stronger," adapting to its environment and even self-correcting issues.

For a deeper dive into the architectural concepts, including "Automorphic System," "Tower of Reflection," and "Grep2Code Morphism," please see [ARCHITECTURE.md](../../ARCHITECTURE.md).

## Project Structure Overview

The project is divided into several crates (Rust packages) that work together:

*   **`patch-build-rs` (root crate)**: The main library that orchestrates everything. It defines the core meta-model for representing Rust code.
*   **`mkslop-core`**: Contains the core logic for applying small, targeted code transformations, like fixing problematic format strings.
*   **`mkslop-macros`**: Provides the `mkslop!` procedural macro, which acts as a user-friendly interface to apply the fixes from `mkslop-core`.
*   **`patch-build-rs-macros`**: Provides the `mkbuildrs!` procedural macro, your primary tool for generating `build.rs` content. It internally uses `mkslop!` for automatic patching.

## Setting Up Your Project

To use `patch-build-rs` in your project, you'll typically interact with it as a workspace. Ensure your `Cargo.toml` files are set up correctly.

For example, your main `Cargo.toml` might look like this:

```toml
[workspace]
members = [
    "your_crate",
    "patch-build-rs-macros",
    "mkslop-macros",
    # ... other crates
]

[workspace.dependencies]
# ... common dependencies
```

And your `your_crate/Cargo.toml` (or `examples/Cargo.toml` for our example) would declare dependencies like:

```toml
[dependencies]
patch-build-rs-macros = { path = "../patch-build-rs-macros" }
mkslop-macros = { path = "../mkslop-macros" } # Needed if you directly use mkslop! or if another macro uses it implicitly
```

## Using the `mkbuildrs!` Macro: A Basic Example

The `mkbuildrs!` macro allows you to declaratively define the content of your `build.rs` file. It expands into `println!` statements that Cargo understands.

Let's look at the `examples/basic_build_script.rs` to see it in action.

First, create a new binary crate (e.g., in an `examples` directory) and ensure its `Cargo.toml` has the necessary dependencies:

```toml
# examples/Cargo.toml
[package]
name = "basic-build-script-example"
version = "0.1.0"
edition = "2021"

[dependencies]
patch-build-rs-macros = { path = "../patch-build-rs-macros" }
mkslop-macros = { path = "../mkslop-macros" } # Essential for mkslop! to be found
```

Now, create the `examples/basic_build_script.rs` file:

```rust
// examples/basic_build_script.rs
// This file demonstrates the usage of the mkbuildrs! macro.

use patch_build_rs_macros::mkbuildrs;
use mkslop_macros::mkslop; // Crucial for mkslop! to be in scope for mkbuildrs! expansion

fn main() {
    // Generate build.rs content using mkbuildrs! macro
    mkbuildrs! {
        // Define conditional compilation flags
        check_cfg: "feature", values = ["my_feature", "another_feature"];
        cfg: "my_custom_flag" = "enabled";

        // Declare resource requirements for external build systems (e.g., Nix)
        resource_req: { ram = "8GB", cpu = "4", instance_type = "c5.xlarge" };

        // Declare secrets your build needs access to
        secret_req: ["DATABASE_URL", "API_TOKEN"];

        // Define a systemd service that needs to be active
        systemd_service: "my-app", start_command = "/usr/local/bin/my-app";

        // Specify a userdata script to be executed
        userdata_script: "setup.sh";
    }

    // When this code is run as a build.rs script, Cargo will capture these
    // println! outputs and interpret them as build instructions or metadata.
    // In this example, running it directly will just print to stdout.
}
```

To run this example (assuming it's set up as a binary within your workspace):

```bash
car go run -p basic-build-script-example
```

You will see output similar to what a `build.rs` script would generate, including `cargo:rustc-cfg`, `cargo:rustc-check-cfg`, and `cargo:info` lines. The `mkslop!` macro silently ensures that any problematic format strings within your `cfg` directives are automatically corrected.

## How `mkslop!` Intervenes

When you write `cfg: "my_custom_flag" = "enabled";`, the `mkbuildrs!` macro generates code that looks something like `println!("{}", mkslop!(/* your string here */));`. The `mkslop!` macro then inspects that string literal and, if it matches a known problematic pattern (like `cargo:rustc-cfg={0}"{1}"`), it transforms it into a safe raw string literal (e.g., `r#"cargo:rustc-cfg={0}\"{1}\
```