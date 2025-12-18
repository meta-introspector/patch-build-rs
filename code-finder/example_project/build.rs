use anyhow::{Result, Context};
use code_finder_macros::model_shell_script; // Import the macro used to generate the modules
use find_code_sh; // Import the generated module for find-code.sh
use generate_macro_calls_sh; // Import the generated module for generate-macro-calls.sh

// Embed the JSON models at compile time to generate the Rust modules
const FIND_CODE_SH_MODEL_JSON: &str = include_str!("../../find-code-sh-model.json");
const GENERATE_MACRO_CALLS_SH_MODEL_JSON: &str = include_str!("../../generate-macro-calls-sh-model.json");

model_shell_script! { FIND_CODE_SH_MODEL_JSON }
model_shell_script! { GENERATE_MACRO_CALLS_SH_MODEL_JSON }

fn main() -> Result<()> {
    // Rerun build script if these models or build.rs change
    println!("cargo:rerun-if-changed=../../find-code-sh-model.json");
    println!("cargo:rerun-if-changed=../../generate-macro-calls-sh-model.json");
    println!("cargo:rerun-if-changed=build.rs");

    // Execute the modeled find-code.sh and capture its output
    let find_code_output = find_code_sh::execute().context("Failed to execute modeled find-code.sh")?;

    // Execute the modeled generate-macro-calls.sh using the output from find-code.sh as input
    let macro_calls_rust_code = generate_macro_calls_sh::execute_with_stdin(Some(&find_code_output))
        .context("Failed to execute modeled generate-macro-calls.sh")?;

    // Write the generated macro calls to a temporary file
    let out_dir = std::env::var("OUT_DIR").context("OUT_DIR not set")?;
    let dest_path = std::path::Path::new(&out_dir).join("generated_macro_calls.rs");
    std::fs::write(&dest_path, macro_calls_rust_code.as_bytes()).context("Failed to write generated macro calls to file")?;

    println!("cargo:rustc-env=GENERATED_MACRO_CALLS_PATH={}", dest_path.display());
    println!("cargo:rustc-link-search=native={}", out_dir);

    Ok(())
}
