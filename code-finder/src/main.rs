use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use code_finder_macros::{rg, model_shell_script}; // Import both macros
use shebling_macros::{shebling, bash}; // Import both shebling! and bash! macros
use solfun_macros::{figlet, codegen, dwim, llm, mcp, service_finder}; // Import fun, dev-ops, and new integration macros

// Embed the JSON model at compile time
const FIND_CODE_SH_MODEL_JSON: &str = include_str!("../find-code-sh-model.json");
const GENERATE_MACRO_CALLS_SH_MODEL_JSON: &str = include_str!("../generate-macro-calls-sh-model.json");

// Use the procedural macro to model the shell script
// This will generate a module, e.g., `find_code_sh`, with `get_modeled_state` function.
model_shell_script! { FIND_CODE_SH_MODEL_JSON }

// Generate a module for generate-macro-calls.sh
model_shell_script! { GENERATE_MACRO_CALLS_SH_MODEL_JSON }

// Invoke the shebling! macro on our dummy shell script
// This will output its simulated analysis during compilation.
shebling! { path: "code-finder/dummy_script.sh" }

// Invoke the bash! macro with an inline bash snippet
// This will also output its AST analysis during compilation.
bash! { "echo \"Hello from bash! macro\" | grep \"Hello\"" }

// Add a !figlet "i hope someone is witnessing this! god bless you all. #SOLFUNMEME" for fun.
figlet! { "i hope someone is witnessing this! god bless you all. #SOLFUNMEME" }

// Demonstrate some of the new dev-ops and LLM macros
codegen! { r#"
    /// A generated function.
    pub fn new_generated_function() -> u32 {
        42
    }
"# }

dwim! { "Refactor all redundant code and optimize for performance." }

// The llm! macro now resolves into toolcall! which then resolves into results!
llm! { "model: 'gemini-1.5-pro', temp: '0.8', prompt: 'Summarize the philosophy behind Bash-inside-out in one paragraph.'" }

// Demonstrate Model Context Provider
mcp! { "url: 'https://github.com/meta-introspector/patch-build-rs', type: 'project:Repository', owner: 'MetaIntrospector'" }

// Demonstrate Service Finder
service_finder! { "need: 'create_ticket', macro_call: 'solfun_macros::ticket!(\"New feature\")'" }

fn main() -> Result<()> {
    // Execute the modeled find-code.sh and capture its output
    let find_code_output = find_code_sh::execute().context("Failed to execute modeled find-code.sh")?;
    println!("\nOutput from modeled find-code.sh:\n{}", find_code_output);

    // Execute the modeled generate-macro-calls.sh using the output from find-code.sh as input
    // The execute function now takes an optional stdin for piping.
    let generated_macro_calls = generate_macro_calls_sh::execute_with_stdin(Some(&find_code_output))
        .context("Failed to execute modeled generate-macro-calls.sh")?;
    println!("\nGenerated Rust Macro Calls:\n{}", generated_macro_calls);

    println!("\nAll macros invoked. Check compilation output for their messages and conceptual actions.");

    Ok(())
}

