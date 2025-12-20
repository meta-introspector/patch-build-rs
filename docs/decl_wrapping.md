# Declaration Wrapping with `audit-fix decl-wrap`

This document explains how to use the `audit-fix decl-wrap` subcommand to automatically add `#[decl(...)]` attributes to public declarations (functions, structs, enums, traits) in your Rust codebase. This process helps in building a comprehensive, compile-time registry of your code's structure for introspection and analysis.

## Purpose of `#[decl(...)]` Attributes

The `#[decl(...)]` attribute macro acts as a labeler for your code's public items. When applied, it registers metadata about the declaration (such as its type, name, visibility, and a semantic hash) into a global, compile-time accessible registry. This registry can then be queried by other tools or macros for various purposes, including:

*   **Code Introspection:** Understanding the structure and public API of a crate.
*   **Static Analysis:** Performing checks or generating reports based on declared items.
*   **Code Generation:** Using declaration metadata to inform further code generation.
*   **Formal Verification:** Bridging Rust code elements to formal proofs or models.

## How it Works

The `audit-fix decl-wrap` tool leverages a procedural macro (`#[decl(...)]` from `patch-build-rs-macros`) to achieve this.

1.  **Scanning:** It parses your Rust source files (`.rs` files) to identify public `fn`, `struct`, `enum`, and `trait` declarations.
2.  **Metadata Extraction:** For each identified declaration, it extracts relevant metadata:
    *   `node_type`: The kind of declaration (e.g., "fn", "struct").
    *   `name`: The identifier of the item.
    *   `vis`: The visibility (e.g., "pub").
    *   `hash`: A semantic hash of the item's content, allowing for content-addressability.
    *   `file` and `line`: The file path and line number where the item is declared.
3.  **Attribute Injection:** It then prepends the `#[decl(...)]` attribute with this extracted metadata directly above the original declaration in the source code.

## Usage

The `audit-fix decl-wrap` subcommand offers two primary modes: `dry-run` for previewing changes and the default mode for applying changes.

### 1. Previewing Changes (`--dry-run`)

Before applying any modifications, it's highly recommended to perform a dry-run to see exactly what changes the tool proposes.

To preview the changes for a specific file:

```bash
cargo run --package introspector_core --bin audit-fix -- decl-wrap <path/to/your/file.rs> --dry-run
```

**Example Output:**

```
📁 File: introspector_core/src/lib.rs
🔍 Found 13 public declarations to wrap

┌─ Line 1: 📁 mod `audit_macros`
│ Hash: 911792f5a1db810e
│ Before: pub mod audit_macros;
│ After:
│   #[decl(mod, name = "audit_macros", vis = "pub", hash = "911792f5")]
│   pub mod audit_macros;
└────────────────────────────────────────
... (further declarations) ...
```
*(Note: As of now, `mod` declarations are explicitly skipped by `decl-wrap` to avoid compilation issues related to unstable Rust features. The above example output might differ in a newer version of the tool if `mod` wrapping is enabled.)*

### 2. Applying Changes

To apply the changes directly to a specific file:

```bash
cargo run --package introspector_core --bin audit-fix -- decl-wrap <path/to/your/file.rs>
```

**Example Output:**

```
✅ Wrapped 5 declarations in <path/to/your/file.rs>
```

### 3. Applying to Multiple Files (Iterative Approach)

The `decl-wrap` command currently operates on a single file at a time. To apply it to multiple files or an entire directory, you would need to iterate over the files.

**Example for a `src` directory:**

```bash
find src/ -name "*.rs" -print0 | xargs -0 -I {} cargo run --package introspector_core --bin audit-fix -- decl-wrap {}
```
This command finds all `.rs` files in the `src/` directory and its subdirectories and applies the `decl-wrap` command to each of them.

### Limitations

*   **Module Declarations:** Currently, `#[decl(...)]` attributes are *not* applied to `pub mod` declarations to avoid known compilation issues related to unstable Rust features (`non-inline modules in proc macro input`). The tool explicitly skips `NodeType::Module` items.
*   **Private Items:** Only public items (functions, structs, enums, traits) are currently wrapped.
*   **Macro Definition Site:** The `file!()` and `module_path!()` macros capture information from the point of macro invocation, not necessarily the original file/module of the item being wrapped if the item is re-exported.

## Future Enhancements

*   **Recursive Application:** Implement a mode to apply `decl-wrap` recursively to a directory.
*   **Configuration:** Allow configuration of which item types to wrap or skip.
*   **Improved Line/Column Tracking:** Investigate more robust ways to get precise line/column information within procedural macros if the current approach faces further compiler challenges.