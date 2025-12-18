# Patch Build RS: A Pure Abstraction and Automorphic System for Rust Code Transformation

This repository (`patch-build-rs`) serves as the foundational library and tooling for implementing a highly reflective and self-modifying system for Rust code, inspired by the metaprogramming capabilities of Lean4. Our goal is to enable semantic patching and automated code repair through a "pure abstraction" model.

## The Principle of Pure Abstraction and Automorphic Systems

This document outlines the core philosophical and architectural principle behind our code transformation and analysis engine: achieving a state of pure abstraction through a self-referential, automorphic system inspired by Lean4's metaprogramming kernel.

### 1. The Core Pattern: Quoting as Abstraction

The fundamental process is to treat all code transformations as an act of **quoting**. We take raw, imperative logic (like that found in `rustc` `build.rs` scripts) and lift it into a declarative, purely functional, and formally-grounded meta-model.

-   **The Code**: An imperative script or piece of logic.
-   **The Quoting**: Our macro system (`pure_reflect!`, `patch_build!`, etc.) acts as the quoting mechanism, translating intent into data.
-   **The Data (`Expr`)**: The result is a value in our Lean4-style `Expr` enum. This `Expr` is a structured, manipulable representation of the original logic.

This process allows us to reason about, manipulate, and reduce complex operations as if they were simple data.

### 2. The Tower of Reflection

This abstraction is achieved by climbing a "tower of reflection," or a macro lattice, where each level gains more expressive power to reason about the layer below it.

1.  **Level 0: `stringify!`**: Purely syntactic reflection into a `String`.
2.  **Level 1: `quote!` / `proc_macro`**: Quasi-syntactic reflection into a `TokenStream`.
3.  **Level 2: Numeric Encoding**: Semantic reflection into a `PureProgram`—a set of numbers representing the code's formal structure (Layer 14).
4.  **Level 3: `Expr` Meta-Model**: Total reflection into our Lean4-style `Expr` data structure, which supports formal computation.

The final step is `pure_reflect!`, a macro that ascends this entire tower, taking any Rust code and lifting it into the `Expr` meta-model.

### 3. The Goal: An Automorphic System

When a system's transformation rules can be applied to the rules themselves without loss of structure, it becomes **automorphic**. This is our ultimate goal.

-   **The Transformation (`F`)**: Our `pure_reflect!` macro is the primary transformation function, mapping `Code → Expr`.
-   **Self-Application**: We achieve this state when we can compute `F(F)`—feeding the code of `pure_reflect!` into itself.
-   **Structure Preservation**: The result is an `Expr` that is a perfect, structurally equivalent representation of the `pure_reflect!` macro. The system maps itself *onto itself* as a valid object within its own universe.

This creates a closed, self-referential loop: `Code → Expr → Code`.

By achieving this, the system is no longer just a tool for manipulating external programs; it gains the capacity for true self-analysis and self-modification in a mathematically sound way. It's the point where the observer (the macro system) and the observed (the code) become one and the same, representing the pinnacle of the reflective tower.

---

## Available Macros

The `patch-build-rs` ecosystem provides a suite of powerful procedural macros to help you generate, patch, and optimize your `build.rs` scripts and other Rust code.

### `mkbuildrs!`

This procedural macro allows you to declaratively generate the contents of a `build.rs` script. It's a convenient way to define conditional compilation flags, resource requirements, and other build-time information.

**Example:**

```rust
use patch_build_rs_macros::mkbuildrs;
use mkslop_macros::mkslop;

fn main() {
    mkbuildrs! {
        check_cfg: "feature", values = ["my_feature", "another_feature"];
        cfg: "my_custom_flag" = "enabled";
        resource_req: { ram = "8GB", cpu = "4" };
        secret_req: ["DATABASE_URL"];
    }
}
```

### `mkslop!`

This procedural macro is designed to automatically fix common issues in generated code, with a focus on malformed format strings that can be produced by AI code generators. It is used internally by `mkbuildrs!` but can also be used as a standalone tool.

**Example:**

```rust
use mkslop_macros::mkslop;

fn main() {
    // mkslop! will fix the potentially problematic format string
    println!("{}", mkslop!("cargo:rustc-cfg={0}=\"{1}\""));
}
```

### `#[lru]`

An attribute macro that adds memoization (caching) to a function. This is particularly useful in `build.rs` scripts for caching the results of expensive operations, like querying a database or reading a file, making your builds faster and more deterministic.

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

### `find_nix_rustc`

A procedural macro that helps locate the `rustc` compiler within a Nix environment. This is useful for build scripts and tools that need to inspect the compiler itself.

**Example:**

```rust
use nix2proc_macros::find_nix_rustc;

fn main() {
    let rustc_path = find_nix_rustc!();
    println!("Found rustc at: {:?}", rustc_path);
}
```

---

## Current Status

This repository is under active development. It currently contains:

-   The core `Expr` meta-model for representing Rust code in a Lean4-inspired manner.
-   A `PureProgram` representation for numeric encoding of code.
-   Basic `reduce` functionality for symbolic execution of `Expr` values.
-   The `patch-build-rs-macros` crate, which now includes the `#[lru]` attribute macro for caching function results, particularly useful for optimizing `build.rs` scripts and ensuring deterministic computations.
-   Ongoing work to integrate Nix store querying capabilities to discover Rust compiler source code for various versions, leveraging the `#[lru]` macro for efficient caching of these operations.

Future development will focus on expanding the `pure_reflect!` macro, integrating the patching logic from original `build.rs` files into the `Expr` model, and building the necessary tooling to leverage this automorphic system for semantic code repair, including the comparative analysis of Rust compiler internals across versions.

---

## The Grep2Code Morphism: Bridging External Tools to the Automorphic System

In pursuit of a truly **automorphic system** where all transformations are self-referential and mathematically sound, we recognize that external tooling (like shell commands, `grep`, `ripgrep`, `jq`, `shellcheck`, etc.) represents a significant challenge to a "closed-world" model. These tools operate outside the Rust type system and metaprogramming framework, making formal reasoning and seamless integration difficult.

This section introduces our approach to bringing these "externalities" into the reflective tower, transforming their behavior and data representations into internal, Rust-native constructs using procedural macros and structured data. This forms a "grep2code" morphism, where the output of external tools is formally lifted into our meta-model.

### Bridging External Behaviors with Rust Metaprogramming

Our strategy involves treating the operations of external tools as "vernacular data representations" that can be modeled and then processed by Rust procedural macros. This allows us to:

1.  **Analyze and Describe**: Understand the intent and structure of external scripts/commands.
2.  **Lift to Data**: Represent their operations as structured data (e.g., JSON).
3.  **Generate Rust Code**: Use procedural macros to interpret this structured data, generating Rust code that reflects or simulates the external behavior within our closed system.

### `code-finder`: A Case Study in External Tool Abstraction

The `code-finder` project serves as a concrete example of this morphism in action. Its goal is to find Rust code based on specified keywords, a task typically handled by `ripgrep` or `grep`.

#### `rg!` Macro: Abstracting `ripgrep`

The `rg!` procedural macro (within `code_finder_macros`) is designed to replace direct invocations of the external `ripgrep` command. Instead of shelling out to `rg`, this macro conceptually takes `ripgrep`'s arguments and will, in its full realization, operate on a **Symbolic File System (VFS)**.

*   **Input**: Takes parameters like `pattern: "..."`, `file_type: "..."`.
*   **Transformation**: At compile time, this macro will eventually execute the `ripgrep` command (or an equivalent Rust library) and structure its output into a Rust data type (e.g., `Vec<MatchResult>`).
*   **Output**: Expands to Rust code that makes this structured search result available within the compilation, moving the "search" operation from an external, unpredictable process into an internal, type-safe computation.

This moves `ripgrep`'s role from an external "program" to an internal, metaprogrammable "function" within our Rust codebase.

#### `grast` Tool/Macro: Greppable ASTs for Structural Search

The `grast_core` library and `grast` CLI tool (and the `grast!` macro in `patch-build-rs-macros`) are foundational to making code itself "greppable" in a structured way.

*   **Concept**: Inspired by `gron` (greppable JSON), `grast` converts Rust Abstract Syntax Trees (ASTs) into a "flat triple format" (RDF Turtle), which is inherently line-oriented and thus easily queryable by text-processing tools like `ripgrep`.
*   **Relation to Quoting**: This process directly aligns with our "quoting as abstraction" principle, lifting Rust code (Level 1 `proc_macro` / `TokenStream`) to a structured, "greppable" data representation (`GrastDb` / Turtle string), effectively ascending the **Tower of Reflection**.
*   **Morphism**: The `grast!` macro takes Rust code as input and expands to a `&'static str` literal containing its Turtle representation, providing a direct morphism from Rust AST to a greppable data model.

#### `model_shell_script!` Macro: Encoding External Script Logic

To fully integrate external scripts into our automorphic system, we need to model their logic and behavior within Rust. The `model_shell_script!` procedural macro (also in `code_finder_macros`) achieves this:

*   **Vernacular Data Representation**: It consumes a JSON description of a shell script's operations (e.g., variable definitions, command executions, data flow). This JSON serves as the "Haskell-like IR" that formally captures the script's intent.
*   **Input**: Takes a string literal containing the JSON model of a shell script (e.g., `model_shell_script! { include_str!("find-code-sh-model.json") }`).
*   **Transformation**: At compile time, the macro parses this JSON into strongly-typed Rust data structures.
*   **Output**: Generates Rust code (e.g., a module with functions and data structures) that represents and *simulates* the shell script's logic. Instead of executing external commands, it generates Rust code that *describes* what those commands *would do*, operating on symbolic variables and states.

This directly realizes the idea of treating an external program as a "vernacular data representation... that can be modeled into phi that decodes it into a model," pushing us closer to a **closed-world mathematical model** where the entire system's state and transformations are contained and verifiable within Rust's type system.

### Towards the Automorphic Goal

By systematically replacing external tool invocations with their macro-driven, symbolically modeled Rust counterparts, we significantly advance towards the automorphic ideal. Each step of abstraction reduces reliance on the external environment, increasing determinism, verifiability, and the system's capacity for self-analysis and self-modification at a fundamental, type-theoretic level.

---

## Available Macros

The `patch-build-rs` ecosystem provides a suite of powerful procedural macros to help you generate, patch, and optimize your `build.rs` scripts and other Rust code.

### `mkbuildrs!`

This procedural macro allows you to declaratively generate the contents of a `build.rs` script. It's a convenient way to define conditional compilation flags, resource requirements, and other build-time information.

**Example:**

```rust
use patch_build_rs_macros::mkbuildrs;
use mkslop_macros::mkslop;

fn main() {
    mkbuildrs! {
        check_cfg: "feature", values = ["my_feature", "another_feature"];
        cfg: "my_custom_flag" = "enabled";
        resource_req: { ram = "8GB", cpu = "4" };
        secret_req: ["DATABASE_URL"];
    }
}
```

### `mkslop!`

This procedural macro is designed to automatically fix common issues in generated code, with a focus on malformed format strings that can be produced by AI code generators. It is used internally by `mkbuildrs!` but can also be used as a standalone tool.

**Example:**

```rust
use mkslop_macros::mkslop;

fn main() {
    // mkslop! will fix the potentially problematic format string
    println!("{}", mkslop!("cargo:rustc-cfg={0}=\"{1}\""));
}
```

### `#[lru]`

An attribute macro that adds memoization (caching) to a function. This is particularly useful in `build.rs` scripts for caching the results of expensive operations, like querying a database or reading a file, making your builds faster and more deterministic.

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

### `grast!`

A procedural macro that converts Rust code into an RDF Turtle representation of its Abstract Syntax Tree (AST). This "greppable AST" allows for structured querying of code using text-based tools.

**Example:**

```rust
use patch_build_rs_macros::grast;

let greppable_ast = grast! {
    pub fn calculate_sum(a: i32, b: i32) -> i32 {
        a + b
    }
};
// `greppable_ast` now contains a string in Turtle format representing the AST of `calculate_sum`.
println!("{}", greppable_ast);
```

### `rg!`

A procedural macro that abstracts the functionality of the `ripgrep` (or `grep`) command, bringing its search capabilities into the Rust metaprogramming environment.

**Example:**

```rust
use code_finder_macros::rg;

fn main() {
    // This macro conceptually performs a search and provides its structured results.
    // In its current form, it prints its arguments during compilation.
    rg! { pattern: "fn main", file_type: "rust" };
}
```

### `model_shell_script!`

A procedural macro that takes a JSON description of a shell script's operations and generates Rust code that models its behavior within the Rust type system.

**Example:**

```rust
use code_finder_macros::model_shell_script;

// Assume `find-code-sh-model.json` contains a JSON model of a shell script.
const SHELL_MODEL_JSON: &str = include_str!("../path/to/find-code-sh-model.json");

model_shell_script! { SHELL_MODEL_JSON }

fn main() {
    // This function is generated by the macro, describing the script's operations.
    find_code_sh::execute_modeled_script();
}
```

---

## Current Status

This repository is under active development. It currently contains:

-   The core `Expr` meta-model for representing Rust code in a Lean4-inspired manner.
-   A `PureProgram` representation for numeric encoding of code.
-   Basic `reduce` functionality for symbolic execution of `Expr` values.
-   The `patch-build-rs-macros` crate, including the `#[lru]` attribute macro for caching and the `grast!` macro for AST to greppable format conversion.
-   The new `code-finder` project, demonstrating the abstraction of external shell commands (`rg!` and `model_shell_script!`) into a closed Rust metaprogramming context.
-   Ongoing work to integrate Nix store querying capabilities to discover Rust compiler source code for various versions, leveraging the `#[lru]` macro for efficient caching of these operations.
