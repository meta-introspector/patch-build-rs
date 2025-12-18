use patch_build_rs_macros::mkbuildrs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};


fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=changes/"); // Rerun if any change order file is added/modified

    // Phase 1: Apply change orders
    apply_change_orders();

    // Using mkbuildrs! to define metadata about the patch-build-rs workspace itself.
    mkbuildrs! {
        // A cfg flag to indicate this is the root build of the patching system.
        cfg: "patch_build_rs_workspace" = "true";

        // Declare the components of the patching system as features.
        check_cfg: "feature", values = ["mkslop", "nix2proc", "patch-build-rs-macros"];

        // Define resource requirements for building the entire workspace.
        resource_req: {
            ram = "4GB",
            cpu = "2",
            instance_type = "t3.medium"
        };

        // A placeholder for a secret that might be used in the build process.
        secret_req: ["PATCH_BUILD_RS_CI_TOKEN"];

        generate_lib_rs: r#"
//! The `patch-build-rs` library.
//!
//! This crate provides the core data structures and logic for the
//! Lean4-inspired metaprogramming and semantic patching system.

pub use introspector_core::{Expr, PureProgram};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Tests the beta-reduction logic in `Expr::reduce`.
    #[test]
    fn test_beta_reduction() {
        // Create an expression for `(λx. x) y`
        let identity_fn = Expr::Lam("x".to_string(), Box::new(Expr::Var("x".to_string())));
        let argument = Expr::Const("y".to_string());
        
        let app = Expr::App(Box::new(identity_fn), vec![argument.clone()]);
        
        // The reduced form should be just "y"
        let reduced = app.reduce();
        
        assert_eq!(reduced, argument);
    }

    /// Tests a more complex reduction with substitution.
    #[test]
    fn test_nested_reduction() {
        // Create `(λy. (λx. y)) z`
        let const_fn_inner = Expr::Lam("x".to_string(), Box::new(Expr::Var("y".to_string())));
        let const_fn_outer = Expr::Lam("y".to_string(), Box::new(const_fn_inner));
        let argument = Expr::Const("z".to_string());

        let app = Expr::App(Box::new(const_fn_outer), vec![argument.clone()]);

        // It should reduce to `λx. z`
        let expected_reduced_inner = Expr::Lam("x".to_string(), Box::new(Expr::Const("z".to_string())));
        
        let reduced = app.reduce();

        assert_eq!(reduced, expected_reduced_inner);
    }

    /// Tests the `bind` and `app` meta-model functions.
    #[test]
    fn test_bind_and_apply() {
        // Simulate `my_expr.bind(|x| x.app(vec![const_arg]))`
        
        let my_expr = Expr::PureAttractor(PureProgram {
            set: BTreeSet::from([1, 2, 3]),
            name: "my_program".to_string(),
        });
        
        let const_arg = Expr::Const("config_value".to_string());

        // The meta-level computation
        let meta_computation = my_expr.clone().bind(|x| {
            x.app(vec![const_arg.clone()])
        });

        // Expected structure: bind(my_expr, λx. apply(x, const_arg))
        let expected = Expr::App(
            Box::new(Expr::Const("bind".to_string())),
            vec![
                my_expr.clone(),
                Expr::Lam(
                    "x".to_string(),
                    Box::new(
                        Expr::App(
                            Box::new(Expr::Var("x".to_string())),
                            vec![const_arg.clone()]
                        )
                    )
                )
            ]
        );

        assert_eq!(meta_computation, expected);

        // Note: reducing this `meta_computation` would require a `reduce` implementation
        // that understands the semantics of "bind" and "apply" at the meta-level.
        // The current `reduce` only handles beta-reduction.
    }
}
"#;
    }

    // Phase 2: Write generated lib.rs content
    if let Ok(content) = env::var("GENERATED_LIB_RS_CONTENT") {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        // Adjust path to correctly point to the project's src directory
        // OUT_DIR is typically target/debug/build/crate_name-hash/out
        // So we need to go up three levels to reach the project root
        // and then navigate into src/
        let lib_rs_path = out_dir.join("../../../src/lib.rs");

        fs::write(&lib_rs_path, content).expect("Failed to write generated lib.rs");
        println!("cargo:rerun-if-changed={}", lib_rs_path.display()); // Rerun if lib.rs changes
    }
}

// Helper function to apply change orders
fn apply_change_orders() {
    let changes_dir = Path::new("changes");
    if !changes_dir.exists() {
        return;
    }

    // Collect all change files and sort them
    let mut change_files: Vec<PathBuf> = fs::read_dir(changes_dir)
        .expect("Failed to read changes directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "rs"))
        .collect();
    change_files.sort(); // Sorts by filename, which should handle the numbering

    // Apply each change
    for change_file in change_files {
        println!("cargo:rerun-if-changed={}", change_file.display()); // Rerun if change file changes
        let content = fs::read_to_string(&change_file).expect(&format!("Failed to read change file: {}", change_file.display()));

        let mut current_target_file_path = None;
        let mut current_old_string = Vec::new();
        let mut current_new_string = Vec::new();
        let mut mode = "none"; // "none", "old_string", "new_string"

        let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        for line in content.lines() {
            if line.starts_with("// change: ") {
                if let Some(path) = current_target_file_path {
                    // Apply previous change before starting a new one
                    perform_replacement(&project_root.join(path), &current_old_string.join("\n"), &current_new_string.join("\n"));
                }
                current_target_file_path = Some(PathBuf::from(line.trim_start_matches("// change: ").trim()));
                current_old_string.clear();
                current_new_string.clear();
                mode = "none";
            } else if line.starts_with("// old_string: ") {
                mode = "old_string";
            } else if line.starts_with("// new_string: ") {
                mode = "new_string";
            } else {
                match mode {
                    "old_string" => current_old_string.push(line),
                    "new_string" => current_new_string.push(line),
                    _ => {} // Ignore other lines outside of old_string/new_string blocks
                }
            }
        }

        // Apply the last change in the file
        if let Some(path) = current_target_file_path {
            perform_replacement(&project_root.join(path), &current_old_string.join("\n"), &current_new_string.join("\n"));
        }
    }
}

fn perform_replacement(target_file_path: &Path, old_str: &str, new_str: &str) {
    let file_content = fs::read_to_string(&target_file_path)
        .expect(&format!("Failed to read target file: {}", target_file_path.display()));

    if !file_content.contains(old_str) {
        eprintln!("Warning: old_string not found in {}. Skipping replacement.", target_file_path.display());
        return;
    }

    let replaced_content = file_content.replace(old_str, new_str);
    fs::write(&target_file_path, replaced_content)
        .expect(&format!("Failed to write to target file: {}", target_file_path.display()));
    println!("cargo:rerun-if-changed={}", target_file_path.display()); // Rerun if target file changes
}
