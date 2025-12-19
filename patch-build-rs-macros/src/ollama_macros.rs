// patch-build-rs-macros/src/ollama_macros.rs

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, LitStr, Ident, Token};
use syn::punctuated::Punctuated;

/// Macro for interacting with Ollama.
///
/// Usage:
/// - `ollama!("status")`
/// - `ollama!("model", "gemma3:12b")`
/// - `ollama!("model-info", "gemma3:12b")`
/// - `ollama!("model-version", "gemma3:12b")`
/// - `ollama!("temp", "0.8")`
pub fn ollama_impl(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);

    if args.is_empty() {
        return quote_spanned! {
            proc_macro2::Span::call_site() =>
            compile_error!("ollama! macro requires at least one argument (e.g., \"status\", \"model\", \"model-info\", \"model-version\", \"temp\").")
        }.into();
    }

    let command = args.first().unwrap().value();
    let ollama_base_url = "http://localhost:11434"; // TODO: Make configurable

    match command.as_str() {
        "status" => {
            quote! {
                {
                    println!("cargo:warning=Checking Ollama status at: {}", #ollama_base_url);
                    let url = format!("{}/api/tags", #ollama_base_url);
                    
                    #[cfg(feature = "ollama_reqwest")]
                    {
                        let client = reqwest::blocking::Client::new();
                        match client.get(&url).timeout(std::time::Duration::from_secs(5)).send() {
                            Ok(response) => {
                                if response.status().is_success() {
                                    println!("cargo:warning=Ollama service is running.");
                                    let json_response: serde_json::Value = response.json().unwrap_or_default();
                                    let model_count = json_response["models"].as_array().map_or(0, |v| v.len());
                                    println!("cargo:warning=Found {} models.", model_count);
                                    true
                                } else {
                                    println!("cargo:warning=Ollama service returned non-200 status: {}", response.status().as_u16());
                                    false
                                }
                            }
                            Err(e) => {
                                println!("cargo:warning=Failed to connect to Ollama service: {}", e);
                                false
                            }
                        }
                    }

                    #[cfg(not(feature = "ollama_reqwest"))]
                    {
                        println!("cargo:warning=Ollama status check requires 'ollama_reqwest' feature enabled.");
                        false
                    }
                }
            }.into()
        }
        "model" => {
            if args.len() < 2 {
                return quote_spanned! {
                    proc_macro2::Span::call_site() =>
                    compile_error!("ollama!(\"model\") requires a model name as the second argument.")
                }.into();
            }
            let model_name = args.get(1).unwrap().value();
            quote! {
                {
                    println!("cargo:warning=Checking availability of Ollama model: {}", #model_name);
                    let url = format!("{}/api/tags", #ollama_base_url);

                    #[cfg(feature = "ollama_reqwest")]
                    {
                        let client = reqwest::blocking::Client::new();
                        match client.get(&url).timeout(std::time::Duration::from_secs(5)).send() {
                            Ok(response) => {
                                if response.status().is_success() {
                                    let json_response: serde_json::Value = response.json().unwrap_or_default();
                                    let models = json_response["models"].as_array().map_or(vec![], |v| v.to_vec());
                                    let model_exists = models.iter().any(|m| m["name"].as_str().map_or(false, |name| name.to_lowercase() == #model_name.to_lowercase()));
                                    if model_exists {
                                        println!("cargo:warning=Ollama model {} is available.", #model_name);
                                        true
                                    } else {
                                        println!("cargo:warning=Ollama model {} not found.", #model_name);
                                        false
                                    }
                                } else {
                                    println!("cargo:warning=Ollama service returned non-200 status: {}", response.status().as_u16());
                                    false
                                }
                            }
                            Err(e) => {
                                println!("cargo:warning=Failed to connect to Ollama service: {}", e);
                                false
                            }
                        }
                    }

                    #[cfg(not(feature = "ollama_reqwest"))]
                    {
                        println!("cargo:warning=Ollama model check requires 'ollama_reqwest' feature enabled.");
                        false
                    }
                }
            }.into()
        }
        "model-info" => {
            if args.len() < 2 {
                return quote_spanned! {
                    proc_macro2::Span::call_site() =>
                    compile_error!("ollama!(\"model-info\") requires a model name as the second argument.")
                }.into();
            }
            let model_name = args.get(1).unwrap().value();
            quote! {
                {
                    println!("cargo:warning=Fetching info for Ollama model: {}", #model_name);
                    let url = format!("{}/api/tags", #ollama_base_url);

                    #[cfg(feature = "ollama_reqwest")]
                    {
                        let client = reqwest::blocking::Client::new();
                        match client.get(&url).timeout(std::time::Duration::from_secs(5)).send() {
                            Ok(response) => {
                                if response.status().is_success() {
                                    let json_response: serde_json::Value = response.json().unwrap_or_default();
                                    let models = json_response["models"].as_array().map_or(vec![], |v| v.to_vec());
                                    let model_info = models.iter().find(|m| m["name"].as_str().map_or(false, |name| name.to_lowercase() == #model_name.to_lowercase()));
                                    
                                    if let Some(info) = model_info {
                                        println!("cargo:warning=Ollama model info for {}: {:#?}", #model_name, info);
                                        // You might want to return a structured type here, but for now, just print
                                        true
                                    } else {
                                        println!("cargo:warning=Ollama model {} not found for info lookup.", #model_name);
                                        false
                                    }
                                } else {
                                    println!("cargo:warning=Ollama service returned non-200 status: {}", response.status().as_u16());
                                    false
                                }
                            }
                            Err(e) => {
                                println!("cargo:warning=Failed to connect to Ollama service: {}", e);
                                false
                            }
                        }
                    }

                    #[cfg(not(feature = "ollama_reqwest"))]
                    {
                        println!("cargo:warning=Ollama model info check requires 'ollama_reqwest' feature enabled.");
                        false
                    }
                }
            }.into()
        }
        "model-version" => {
            if args.len() < 2 {
                return quote_spanned! {
                    proc_macro2::Span::call_site() =>
                    compile_error!("ollama!(\"model-version\") requires a model name as the second argument.")
                }.into();
            }
            let model_name = args.get(1).unwrap().value();
            quote! {
                {
                    println!("cargo:warning=Extracting version for Ollama model: {}", #model_name);
                    // Assuming version is part of the name like "gemma3:12b" -> "12b"
                    let parts: Vec<&str> = #model_name.split(':').collect();
                    if parts.len() > 1 {
                        let version = parts[1].to_string();
                        println!("cargo:warning=Ollama model {} version: {}", #model_name, version);
                        version
                    } else {
                        println!("cargo:warning=Version not found in model name: {}", #model_name);
                        "".to_string()
                    }
                }
            }.into()
        }
        "temp" => {
            if args.len() < 2 {
                return quote_spanned! {
                    proc_macro2::Span::call_site() =>
                    compile_error!("ollama!(\"temp\") requires a temperature value as the second argument.")
                }.into();
            }
            let temp_value = args.get(1).unwrap().value();
            quote! {
                {
                    println!("cargo:warning=Setting Ollama temperature to: {}", #temp_value);
                    // For now, this just prints. Actual usage would be in a generation macro.
                    #temp_value
                }
            }.into()
        }
        _ => {
            quote_spanned! {
                proc_macro2::Span::call_site() =>
                compile_error!("Unsupported ollama! command. Available commands: \"status\", \"model\", \"model-info\", \"model-version\", \"temp\".")
            }.into()
        }
    }
}
