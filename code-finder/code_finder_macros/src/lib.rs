extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, LitStr, Ident, Token,
    parse::{Parse, ParseStream},
    Result as SynResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- rg! macro definition ---

// Represents the arguments for the rg! macro
struct RgArgs {
    pattern: LitStr,
    _comma1: Option<Token![,]>,
    file_type_key: Option<Ident>, // "file_type"
    _colon1: Option<Token![:]>,
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

    let expanded = if let Some(ft) = file_type_value {
        quote! {
            // Placeholder: In a real scenario, this would execute 'rg' and return structured data.
            println!("rg! macro invoked with pattern: \"{}\", file_type: \"{}\"", #pattern_value, #ft);
        }
    } else {
        quote! {
            // Placeholder: In a real scenario, this would execute 'rg' and return structured data.
            println!("rg! macro invoked with pattern: \"{}\"", #pattern_value);
        }
    };

    expanded.into()
}

// --- model_shell_script! macro definition ---

#[derive(Debug, Deserialize)]
struct ShellScriptModel {
    script_name: String,
    description: String,
    operations: Vec<Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")] // This allows deserializing based on the "type" field
enum Operation {
    #[serde(rename = "variable_definition")]
    VariableDefinition {
        name: String,
        value_source: String, // e.g., "literal_string", "command_output"
        #[serde(default)]
        content: Option<String>, // For literal_string
        #[serde(default)]
        command: Option<String>, // For command_output
        #[serde(default)]
        command_args: Option<Vec<String>>, // For command_output
        #[serde(default)]
        input_stream_from: Option<InputStreamSource>, // For command_output
    },
    #[serde(rename = "command_execution")]
    CommandExecution {
        program: String,
        arguments: Vec<CommandArgument>,
        output_destination: String, // e.g., "stdout"
    },
}

#[derive(Debug, Deserialize)]
struct InputStreamSource {
    #[serde(rename = "type")]
    source_type: String, // e.g., "variable"
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Try to deserialize as one variant, then another
enum CommandArgument {
    Literal(String),
    Variable {
        source: String, // Must be "variable"
        name: String,
    },
}


#[proc_macro]
pub fn model_shell_script(input: TokenStream) -> TokenStream {
    let json_str = parse_macro_input!(input as LitStr);
    let json_value = json_str.value();

    let model: ShellScriptModel = match serde_json::from_str(&json_value) {
        Ok(m) => m,
        Err(e) => {
            return quote_spanned! {json_str.span() =>
                compile_error!(format!("Failed to parse shell script model JSON: {}", e));
            }
            .into();
        }
    };

    let script_name_ident = Ident::new(&model.script_name.replace('.', "_"), json_str.span());
    let description = &model.description;

    let mut operations_code = Vec::new();

    for op in &model.operations {
        match op {
            Operation::VariableDefinition { name, value_source, content, command, command_args, input_stream_from } => {
                let var_name = Ident::new(&name, json_str.span());
                let var_type = Ident::new("String", json_str.span()); // Assume string for simplicity

                let value_expr = match value_source.as_str() {
                    "literal_string" => {
                        let content_val = content.as_ref().map_or("".to_string(), |s| s.clone());
                        quote! { #content_val.to_string() }
                    },
                    "command_output" => {
                        let cmd_program = command.as_ref().map_or("unknown_cmd".to_string(), |s| s.clone());
                        let args_str = command_args.as_ref().map_or("".to_string(), |args| args.join(" "));
                        let input_src = if let Some(input_stream) = input_stream_from {
                            let src_name = Ident::new(&input_stream.name, json_str.span());
                            quote! { format!("(input from {})", #src_name) }
                        } else {
                            quote! { "".to_string() }
                        };
                        quote! {
                            format!("Output of: {}{}{}", #cmd_program, #args_str, #input_src)
                        }
                    },
                    _ => quote! { "unsupported_value_source".to_string() },
                };
                operations_code.push(quote! {
                    #[allow(dead_code)]
                    let #var_name: #var_type = #value_expr;
                    println!("  Variable {}: {}", stringify!(#var_name), #var_name);
                });
            },
            Operation::CommandExecution { program, arguments, output_destination } => {
                let program_str = program;
                let args_vec = arguments.iter().map(|arg| {
                    match arg {
                        CommandArgument::Literal(s) => quote! { #s.to_string() },
                        CommandArgument::Variable { source: _, name } => {
                            let var_ident = Ident::new(&name, json_str.span());
                            quote! { #var_ident.clone() }
                        },
                    }
                }).collect::<Vec<_>>();
                operations_code.push(quote! {
                    println!("  Executing command: {} with args {:?}", #program_str, vec![#(#args_vec),*]);
                    println!("  Output destination: {}", #output_destination);
                });
            },
        }
    }

    let expanded = quote! {
        mod #script_name_ident {
            #[allow(unused_imports)]
            use super::*; // Allow access to outside definitions if needed

            #[derive(Debug, Clone)] // Add Clone for static context
            pub struct ShellScriptRepresentation {
                pub name: &'static str,
                pub description: &'static str,
            }

            // Using `static` and `Lazy` for a single global instance
            use once_cell::sync::Lazy;

            pub static MODEL_METADATA: Lazy<ShellScriptRepresentation> = Lazy::new(|| {
                ShellScriptRepresentation {
                    name: #script_name_ident.name,
                    description: #description,
                }
            });

            pub fn execute_modeled_script() {
                println!("Modeling script: {}", MODEL_METADATA.name);
                println!("Description: {}", MODEL_METADATA.description);
                #(#operations_code)*
            }
        }
    };

    expanded.into()
}
