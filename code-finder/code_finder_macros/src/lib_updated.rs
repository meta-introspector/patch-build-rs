extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input,
    LitStr,
    Ident,
    Token,
    parse::{Parse, ParseStream},
    Result as SynResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use brush_parser::{parse, ast::{TopLevelCommand, Command, Expression, Word, Redirect}};


// --- Structs for ripgrep JSON output ---
#[derive(Debug, Deserialize, Serialize)]
pub struct RgMatch {
    #[serde(rename = "type")]
    pub kind: String, // "match"
    pub data: RgMatchData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgMatchData {
    pub path: RgPath,
    pub lines: RgLines,
    #[serde(default)] // lines can be empty if --only-matching is used.
    pub line_number: Option<usize>,
    pub absolute_offset: usize,
    pub submatches: Vec<RgSubmatch>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgPath {
    #[serde(rename = "text")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RgLines {
    Text {
        #[serde(rename = "text")]
        value: String,
    },
    Bytes {
        #[serde(rename = "bytes")]
        value: String, // Base64 encoded
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgSubmatch {
    #[serde(rename = "match")]
    pub content: RgMatchContent,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RgMatchContent {
    Text {
        #[serde(rename = "text")]
        value: String,
    },
    Bytes {
        #[serde(rename = "bytes")]
        value: String, // Base64 encoded
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgBegin {
    #[serde(rename = "type")]
    pub kind: String, // "begin"
    pub data: RgBeginData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgBeginData {
    pub path: RgPath,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgEnd {
    #[serde(rename = "type")]
    pub kind: String, // "end"
    pub data: RgEndData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgEndData {
    pub path: RgPath,
    pub stats: RgStats,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgStats {
    pub elapsed_ns: usize,
    pub matches: usize,
    pub bytes_read: usize,
    pub bytes_elapsed: usize,
    pub lines_elapsed: usize,
    pub searches: usize,
    pub time_total_ns: usize,
}

// Top-level enum to deserialize any ripgrep JSON line
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)] // Try to deserialize as one variant, then another
pub enum RgMessage {
    RgMatch(RgMatch),
    RgBegin(RgBegin),
    RgEnd(RgEnd),
    // context and summary messages are currently ignored but can be added if needed
    #[serde(other)]
    Unknown,
}


// --- rg! macro definition ---

// Represents the arguments for the rg! macro
struct RgArgs {
    pattern: LitStr,
    _comma1: Option<Token![,]>
    file_type_key: Option<Ident>, // "file_type"
    _colon1: Option<Token![:]>
    file_type: Option<LitStr>,
}

impl Parse for RgArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let pattern_key: Ident = input.parse()?;
        if pattern_key != "pattern" {
            return Err(input.error("Expected 'pattern' as the first argument"));
        }
        input.parse::<Token![:]>()?;
        let pattern = input.parse()?;

        let mut _comma1 = None;
        let mut file_type_key = None;
        let mut _colon1 = None;
        let mut file_type = None;

        if !input.is_empty() {
            _comma1 = Some(input.parse()?);
            if !input.is_empty() {
                file_type_key = Some(input.parse()?);
                if file_type_key.as_ref().map_or(false, |id| id != "file_type") {
                    return Err(input.error("Expected 'file_type' as the second argument if present"));
                }
                _colon1 = Some(input.parse()?);
                file_type = Some(input.parse()?);
            }
        }

        Ok(RgArgs {
            pattern,
            _comma1,
            file_type_key,
            _colon1,
            file_type,
        })
    }
}

#[proc_macro]
pub fn rg(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RgArgs);

    let pattern_value = args.pattern.value();
    let file_type_value = args.file_type.map(|lit| lit.value());

    let mut command = std::process::Command::new("rg");
    command.arg("--json").arg(&pattern_value);

    if let Some(ft) = &file_type_value {
        command.arg("-t").arg(ft);
    }

    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            return quote_spanned! {args.pattern.span() =>
                compile_error!(format!("Failed to execute ripgrep: {}", e));
            }
            .into();
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return quote_spanned! {args.pattern.span() =>
            compile_error!(format!("ripgrep command failed: {}", #stderr));
        }
        .into();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut matches = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<RgMessage>(line) {
            Ok(RgMessage::RgMatch(m)) => matches.push(m),
            Ok(_) => { /* Ignore other message types for now */ },
            Err(e) => {
                return quote_spanned! {args.pattern.span() =>
                    compile_error!(format!("Failed to parse ripgrep JSON output line '{}': {}", #line, e));
                }
                .into();
            }
        }
    }

    let matches_json = match serde_json::to_string(&matches) {
        Ok(json) => json,
        Err(e) => {
            return quote_spanned! {args.pattern.span() =>
                compile_error!(format!("Failed to serialize matches to JSON: {}", e));
            }
            .into();
        }
    };

    // The macro expands to a block that parses the JSON string at runtime
    // to produce a Vec<RgMatch>.
    let expanded = quote! {
        {
            #[allow(unused_imports)]
            use serde_json;
            #[allow(unused_imports)]
            use code_finder_macros::{RgMatch, RgMessage}; // Import necessary structs

            let json_data = #matches_json;
            serde_json::from_str::<Vec<RgMatch>>(json_data).expect("Failed to deserialize ripgrep matches")
        }
    };

    expanded.into()
}

// --- Symbolic representation structs ---
#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for generated code
pub enum SymbolicExpression {
    Literal(String),
    Variable(String),
    CommandOutput {
        program: String,
        args: Vec<SymbolicExpression>,
        input: Option<Box<SymbolicExpression>>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
pub struct SymbolicVariableDefinition {
    pub name: String,
    pub value: SymbolicExpression,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
pub struct SymbolicCommandExecution {
    pub program: String,
    pub arguments: Vec<SymbolicExpression>,
    pub output_destination: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
pub enum SymbolicShellOperation {
    VariableDefinition(SymbolicVariableDefinition),
    CommandExecution(SymbolicCommandExecution),
    GenerateRustMacros(SymbolicGenerateRustMacros),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolicGenerateRustMacros {
    pub input_source: String,
    pub macro_name: String,
    pub mapping_rules: Vec<SymbolicMacroMappingRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolicMacroMappingRule {
    pub field: String,
    pub json_path: String,
    pub escape_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone
pub struct ShellScriptSymbolicState {
    pub script_name: String,
    pub description: String,
    pub operations: Vec<SymbolicShellOperation>,
}


// --- model_shell_script! macro definition ---

// Represents the arguments for the model_shell_script! macro
struct ModelShellScriptArgs {
    path_key: Ident, // "path"
    _colon1: Token![:],
    path: LitStr,
}

impl Parse for ModelShellScriptArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let path_key: Ident = input.parse()?;
        if path_key != "path" {
            return Err(input.error("Expected 'path' as the argument"));
        }
        let _colon1 = input.parse()?;
        let path = input.parse()?;
        Ok(ModelShellScriptArgs { path_key, _colon1, path })
    }
}


#[proc_macro]
pub fn model_shell_script(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as ModelShellScriptArgs);
    let script_path = args.path.value();
    let span = args.path.span();

    let full_path = PathBuf::from(&script_path);

    let script_content = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            return quote_spanned! {span =>
                compile_error!(format!("Failed to read shell script at {}: {}", #script_path, #e));
            }
            .into();
        }
    };

    let ast = match parse(&script_content) {
        Ok(ast) => ast,
        Err(e) => {
            return quote_spanned! {span =>
                compile_error!(format!("Failed to parse shell script at {}: {}", #script_path, #e));
            }
            .into();
        }
    };

    let script_name = full_path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown_script");
    let script_name_ident = Ident::new(&script_name.replace('.', "_"), span);
    let description = format!("Modeled shell script from {}", script_path);

    let mut symbolic_operations = Vec::new();

    for command in ast.commands {
        match command {
            TopLevelCommand::Simple(simple_command) => {
                let program_str = simple_command.leading_word.map_or("".to_string(), |w| w.data);
                let mut args_vec = Vec::new();
                for arg in simple_command.words {
                    match arg.data {
                        Expression::Bare { text } => args_vec.push(quote! { SymbolicExpression::Literal(#text.to_string()) }),
                        _ => args_vec.push(quote! { SymbolicExpression::Literal(format!("{:?}", #arg)) }), // Fallback for complex expressions
                    }
                }
                // Simplified: assuming all simple commands are CommandExecution to stdout
                symbolic_operations.push(quote! {
                    SymbolicShellOperation::CommandExecution(SymbolicCommandExecution {
                        program: #program_str.to_string(),
                        arguments: vec![#(#args_vec),*],
                        output_destination: "stdout".to_string(), // Default for simple commands
                    })
                });
            },
            TopLevelCommand::Pipeline(pipeline) => {
                // For pipeline, model each command in the pipeline
                for cmd in pipeline.commands {
                    let program_str = cmd.leading_word.map_or("".to_string(), |w| w.data);
                    let mut args_vec = Vec::new();
                    for arg in cmd.words {
                        match arg.data {
                            Expression::Bare { text } => args_vec.push(quote! { SymbolicExpression::Literal(#text.to_string()) }),
                            _ => args_vec.push(quote! { SymbolicExpression::Literal(format!("{:?}", #arg)) }),
                        }
                    }
                    symbolic_operations.push(quote! {
                        SymbolicShellOperation::CommandExecution(SymbolicCommandExecution {
                            program: #program_str.to_string(),
                            arguments: vec![#(#args_vec),*],
                            output_destination: "piped".to_string(), // Indicates output is piped
                        })
                    });
                }
            },
            TopLevelCommand::Assignment(assignment) => {
                let var_name = assignment.var_name.data;
                let value_expr = match assignment.value.data {
                    Expression::Bare { text } => quote! { SymbolicExpression::Literal(#text.to_string()) },
                    _ => quote! { SymbolicExpression::Literal(format!("{:?}", #assignment.value)) }, // Fallback
                };
                symbolic_operations.push(quote! {
                    SymbolicShellOperation::VariableDefinition(SymbolicVariableDefinition {
                        name: #var_name.to_string(),
                        value: #value_expr,
                    })
                });
            },
            _ => {
                // Handle other top-level commands as needed, e.g., If, While, FunctionDef
                // For now, represent them as generic command execution
                let cmd_debug = format!("{:#?}", command);
                symbolic_operations.push(quote! {
                    SymbolicShellOperation::CommandExecution(SymbolicCommandExecution {
                        program: "unsupported_shell_command".to_string(),
                        arguments: vec![SymbolicExpression::Literal(#cmd_debug.to_string())],
                        output_destination: "stderr".to_string(),
                    })
                });
            }
        }
    }

    let expanded = quote! {
        mod #script_name_ident {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use once_cell::sync::Lazy;

            pub static MODELLED_SCRIPT_STATE: Lazy<ShellScriptSymbolicState> = Lazy::new(|| {
                ShellScriptSymbolicState {
                    script_name: #script_name.to_string(),
                    description: #description.to_string(),
                    operations: vec![
                        #(#symbolic_operations),*
                    ],
                }
            });

            pub fn get_modeled_state() -> &'static ShellScriptSymbolicState {
                &MODELLED_SCRIPT_STATE
            }

            // Generated function to execute the modeled script.
            pub fn execute() -> Result<String, anyhow::Error> {
                Self::execute_with_stdin(None)
            }

            // Generated function to execute the modeled script with provided stdin data.
            pub fn execute_with_stdin(stdin_data: Option<&str>) -> Result<String, anyhow::Error> {
                use std::collections::HashMap;
                use std::process::{Command, Stdio};
                use std::io::{Write, Read};
                use anyhow::{Result, Context};
                use serde_json::Value; // Needed for GenerateRustMacros handling

                let mut variables: HashMap<String, String> = HashMap::new();
                let mut stdout_buffer = String::new();
                let operations = &MODELLED_SCRIPT_STATE.operations;

                // Handle initial stdin if provided. This is primarily for the first command in a pipeline.
                let mut current_stdin_for_next_command: Option<String> = stdin_data.map(|s| s.to_string());


                for op in operations {
                    match op {
                        SymbolicShellOperation::VariableDefinition(var_def) => {
                            let var_name = &var_def.name;
                            let var_value = match &var_def.value {
                                SymbolicExpression::Literal(s) => s.clone(),
                                SymbolicExpression::Variable(name) => variables.get(name).cloned().unwrap_or_default(),
                                SymbolicExpression::CommandOutput { program, args, input } => {
                                    let mut cmd = Command::new(program);
                                    for arg_expr in args {
                                        match arg_expr {
                                            SymbolicExpression::Literal(s) => { cmd.arg(s); },
                                            SymbolicExpression::Variable(name) => { cmd.arg(variables.get(name).cloned().unwrap_or_default()); },
                                            _ => { return Err(anyhow::anyhow!("Unsupported argument type for CommandOutput: {:?}", arg_expr)); }
                                        }
                                    }

                                    if let Some(input_expr) = input {
                                        match input_expr.as_ref() {
                                            SymbolicExpression::Variable(name) => {
                                                let input_data = variables.get(name).cloned().unwrap_or_default();
                                                cmd.stdin(Stdio::piped());
                                                let mut child = cmd.spawn().context(format!("Failed to spawn command {}", program))?;
                                                let mut stdin = child.stdin.take().context("Failed to open stdin")?;
                                                stdin.write_all(input_data.as_bytes()).context("Failed to write to stdin")?;
                                                drop(stdin); // Close stdin to signal EOF
                                                let output = child.wait_with_output().context(format!("Failed to wait for command {}", program))?;
                                                if !output.status.success() {
                                                    return Err(anyhow::anyhow!("Command '{}' failed: {}", program, String::from_utf8_lossy(&output.stderr)));
                                                }
                                                String::from_utf8_lossy(&output.stdout).to_string()
                                            },
                                            _ => { return Err(anyhow::anyhow!("Unsupported input type for CommandOutput: {:?}", input_expr)); }
                                        }
                                    } else {
                                        let output = cmd.output().context(format!("Failed to execute command {}", program))?;
                                        if !output.status.success() {
                                            return Err(anyhow::anyhow!("Command '{}' failed: {}", program, String::from_utf8_lossy(&output.stderr)));
                                        }
                                        String::from_utf8_lossy(&output.stdout).to_string()
                                    }
                                },
                            };
                            variables.insert(var_name.clone(), var_value);
                        },
                        SymbolicShellOperation::CommandExecution(cmd_exec) => {
                            let mut cmd = Command::new(&cmd_exec.program);
                            for arg_expr in &cmd_exec.arguments {
                                match arg_expr {
                                    SymbolicExpression::Literal(s) => { cmd.arg(s); },
                                    SymbolicExpression::Variable(name) => { cmd.arg(variables.get(name).cloned().unwrap_or_default()); },
                                    _ => { return Err(anyhow::anyhow!("Unsupported argument type for CommandExecution: {:?}", arg_expr)); }
                                }
                            }

                            if current_stdin_for_next_command.is_some() {
                                cmd.stdin(Stdio::piped());
                            }

                            let mut child = cmd.spawn().context(format!("Failed to spawn command {}", cmd_exec.program))?;

                            if let Some(data) = current_stdin_for_next_command.take() { // Consume stdin data for this command
                                let mut stdin_writer = child.stdin.take().context("Failed to open stdin for child")?;
                                stdin_writer.write_all(data.as_bytes()).context("Failed to write to child stdin")?;
                                drop(stdin_writer); // Close stdin to signal EOF
                            }

                            let output = child.wait_with_output().context(format!("Failed to wait for command {}", cmd_exec.program))?;
                            if !output.status.success() {
                                return Err(anyhow::anyhow!("Command '{}' failed: {}", cmd_exec.program, String::from_utf8_lossy(&output.stderr)));
                            }
                            // If output is destined for stdout (or piped), store it.
                            if cmd_exec.output_destination == "stdout" || cmd_exec.output_destination == "piped" {
                                stdout_buffer.push_str(&String::from_utf8_lossy(&output.stdout));
                            }
                            // If output is piped, this becomes the stdin for the next command.
                            if cmd_exec.output_destination == "piped" {
                                current_stdin_for_next_command = Some(String::from_utf8_lossy(&output.stdout).to_string());
                            } else {
                                current_stdin_for_next_command = None; // Reset stdin for next command if not piped
                            }
                        },
                        SymbolicShellOperation::GenerateRustMacros(gen_macros) => {
                            if gen_macros.input_source == "stdin_json_rg_output" {
                                let input_json_lines = if let Some(data) = current_stdin_for_next_command.take() {
                                    data
                                } else {
                                    stdout_buffer.clone() // Fallback to accumulated stdout if no direct stdin
                                };
                                stdout_buffer.clear(); // Clear buffer as it's consumed after being passed as stdin

                                let mut generated_macro_calls = String::new();
                                generated_macro_calls.push_str(&format!("// Generated by {} based on JSON input\n", gen_macros.macro_name));

                                for line in input_json_lines.lines() {
                                    if line.trim().is_empty() { continue; }
                                    let v: Value = serde_json::from_str(line).context(format!("Failed to parse JSON line: {}", line))?;
                                    if v["type"] != "match" { continue; }

                                    generated_macro_calls.push_str(&format!(