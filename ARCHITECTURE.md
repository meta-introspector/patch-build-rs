# Architecture of the `patch-build-rs` Ecosystem

This document formalizes the high-level concepts that underpin the `patch-build-rs` ecosystem, providing practical definitions for its core architectural principles.

## Automorphic System

An **Automorphic System** in the context of `patch-build-rs` refers to the system's inherent ability to apply its primary code-to-data transformation macro, `pure_reflect!`, to its own source code. This process results in a structurally equivalent `Expr` representation of the macro itself. This self-application closes the fundamental `Code → Expr → Code` loop, thereby enabling formal self-analysis, introspection, and self-modification of the system's own components within its defined meta-model. This ensures that the system can reason about and transform its own logic in a mathematically sound and verifiable manner.

## Tower of Reflection

The **Tower of Reflection** describes the four-level macro lattice utilized by the `patch-build-rs` system to systematically lift Rust code into a formal, manipulable model. Each level in the tower represents an increasing degree of reflection and abstraction:

*   **Level 0: `stringify!`**: This is the most basic level, performing purely syntactic reflection by converting code into a `String` literal.
*   **Level 1: `quote!` / `proc_macro`**: At this level, code undergoes quasi-syntactic reflection, being transformed into a `TokenStream`. This allows for more structured manipulation than a raw string.
*   **Level 2: Numeric `PureProgram` Encoding**: This level introduces semantic reflection, encoding the code's formal structure into a `PureProgram`—a set of numerical representations. This is a crucial step towards a machine-readable and computable form.
*   **Level 3: `Expr` Meta-Model**: The pinnacle of the tower, this level achieves total reflection. Code is transformed into a Lean4-style `Expr` data structure, which is a rich, symbolic meta-model capable of formal computation, reasoning, and manipulation.

The `pure_reflect!` macro is the orchestrator that ascends this entire tower, taking any arbitrary Rust code as input and lifting it into the comprehensive `Expr` meta-model for deep analysis and transformation.

## Grep2Code Morphism

The **Grep2Code Morphism** is an architectural pattern within `patch-build-rs` designed to formally integrate external tool operations into the system's "closed-world" Rust model. External tools, such as `ripgrep`, `grep`, or shell scripts, traditionally operate outside the Rust type system, making their outputs and behaviors opaque to formal analysis.

This morphism addresses this challenge by:

1.  **Modeling External Behavior**: Macros like `rg!` and `model_shell_script!` are used to capture the intent and expected behavior of external commands.
2.  **Transforming Output to Internal Constructs**: The output or conceptual action of these external tools is transformed into internal, type-safe Rust data structures and constructs. For instance, `ripgrep`'s search results are structured into Rust types, or a shell script's logic is translated into a symbolic Rust model.

By applying this pattern, `patch-build-rs` converts the "untyped" operations of the external environment into a form that can be reasoned about, manipulated, and verified within its own reflective, automorphic system. This bridges the gap between external system interactions and the internal formal model, ensuring architectural consistency and enhancing the system's overall verifiability.

## Architectural Flow Diagram (Placeholder)

[Diagram illustrating the flow of a standard Rust `build.rs` script as it is "quoted," lifted up the tower of reflection into an `Expr` representation, and made available for symbolic manipulation goes here.]