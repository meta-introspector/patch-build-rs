# Current Plan and Learnings

## What I Learned About the Codebase

### Core Architecture
- **Automorphic System**: Implements a "tower of reflection" where code transformations can be applied to themselves
- **`Expr` Type**: Lean4-inspired AST representation for total code reflection into manipulable data
- **Grast System**: Converts Rust AST to greppable RDF Turtle format for structural search
- **Grep2Code Morphism**: Abstracts external tools (ripgrep, shell scripts) into Rust macros

### Key Existing Macros
- `pure_reflect!` - Total reflection of code into `Expr` meta-model
- `grast!` - AST to greppable format conversion  
- `mkbuildrs!` - Declarative build.rs generation
- `rg!` - Abstracts ripgrep into compile-time search
- `model_shell_script!` - Models shell scripts as Rust data structures
- `solfun_macros` - Fun/dev-ops macros (`figlet!`, `codegen!`, `ticket!`)

## Current Plan

### Phase 1: Rust Source Integration
1. Find rustc in Nix store
2. Extract Rust compiler source version/commit
3. Download matching source to `submodules/rust/`
4. Create mounting/linking mechanism

### Phase 2: Macro Injection System
1. Enhance `RustInNix!` macro to:
   - Query Nix store for Rust installations
   - Download/extract matching source
   - Inject our macros into all `build.rs` files
   - Replace `bootstrap.py` with macro-driven version

### Phase 3: Autowrap Integration
1. Integrate `autowrap!` macros into Rust compiler build
2. Wrap compilation errors with auto-fixing logic
3. Create TOML-driven refactoring pipeline

### Phase 4: Self-Modification
1. Apply `pure_reflect!` to the macro system itself
2. Achieve automorphic state where transformation rules apply to themselves
3. Enable true self-analysis and self-modification

## Next Immediate Steps
1. Get rustc version/source from Nix environment
2. Mount source into `submodules/rust/`
3. Test macro injection into Rust compiler build process

## Notes
The system is already quite sophisticated - we're extending it to make the Rust compiler itself part of the automorphic transformation system.
