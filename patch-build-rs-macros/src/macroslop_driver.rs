use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Ident, LitInt, LitStr, Token};
use std::collections::HashMap;

/// Represents the arguments passed to `mkmacroslop!`.
struct MkMacroSlopArgs {
    output_dir: LitStr,
    chunk_size: LitStr,
    // For now, `extract` is just a placeholder, but could be parsed into a more complex structure.
    extract: Vec<Ident>, 
}

impl Parse for MkMacroSlopArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut output_dir = None;
        let mut chunk_size = None;
        let mut extract = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            if key == "output_dir" {
                output_dir = Some(input.parse()?);
            } else if key == "chunk_size" {
                chunk_size = Some(input.parse()?);
            } else if key == "extract" {
                let content;
                syn::bracketed!(content in input);
                let extracted_items: syn::punctuated::Punctuated<Ident, Token![,]> =
                    content.parse_terminated(Ident::parse, Token![,])?;
                extract = extracted_items.into_iter().collect();
            } else {
                return Err(input.error(format!("unexpected argument: {}", key)));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MkMacroSlopArgs {
            output_dir: output_dir.ok_or_else(|| input.error("missing 'output_dir' argument"))?,
            chunk_size: chunk_size.ok_or_else(|| input.error("missing 'chunk_size' argument"))?,
            extract,
        })
    }
}

/// A procedural macro to construct a Rust program for source code analysis.
///
/// This macro generates a `main` function that will:
/// 1. Traverse a target directory for Rust source files.
/// 2. Parse them using `syn`.
/// 3. Extract specified data (e.g., public declarations).
/// 4. Serialize the report into chunked JSON files.
///
/// Usage:
/// ```ignore
/// mkmacroslop! {
///     output_dir: "analysis_reports",
///     chunk_size: "2MB",
///     extract: [public_decls, dependencies, function_bodies]
/// }
/// ```
#[proc_macro]
pub fn mkmacroslop(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as MkMacroSlopArgs);

    let output_dir = args.output_dir;
    let chunk_size = args.chunk_size;
    let extract_items = args.extract; // For future use in customizing extraction logic

    let generated_code = quote! {
        extern crate walkdir;
        extern crate syn;
        extern crate quote;
        extern crate serde;
        extern crate serde_json;
        extern crate anyhow;
        extern crate toml; // For parsing Cargo.toml

        use std::fs;
        use std::path::{Path, PathBuf};
        use serde::{Serialize, Deserialize};
        use anyhow::Context;
        use std::collections::HashMap;

        // --- Data Structures for Report ---

        #[derive(Serialize, Deserialize, Debug)]
        pub struct FunctionDetails {
            pub parameters: Vec<String>,
            pub return_type: Option<String>,
            pub qualifiers: Vec<String>, // e.g., "async", "const", "unsafe"
            pub body_snippet: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct StructDetails {
            pub fields: HashMap<String, String>, // field_name -> field_type
            pub generics: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct EnumDetails {
            pub variants: Vec<String>,
            pub generics: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct TraitDetails {
            pub methods: Vec<FunctionDetails>, // Only method signatures
            pub associated_types: Vec<String>,
            pub generics: Option<String>,
            pub supertraits: Vec<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct MacroDetails {
            pub kind: String, // e.g., "macro_rules", "proc_macro", "proc_macro_attribute", "proc_macro_derive"
            pub definition_snippet: String, // The full token stream of the macro definition
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct PublicDeclaration {
            pub name: String,
            pub kind: String, // "fn", "struct", "enum", "const", "static", "trait", "mod", "type", "macro"
            pub signature: String, // A string representation of the declaration's signature
            pub documentation: Option<String>, // Doc comments
            pub function_details: Option<FunctionDetails>,
            pub struct_details: Option<StructDetails>,
            pub enum_details: Option<EnumDetails>,
            pub trait_details: Option<TraitDetails>,
            pub macro_details: Option<MacroDetails>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct CrateReport {
            pub crate_name: String,
            pub path: PathBuf,
            pub public_decls: Vec<PublicDeclaration>,
            pub dependencies: HashMap<String, String>, // name -> version
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct AnalysisReport {
            pub compiler_version: String,
            pub crates: Vec<CrateReport>,
        }

        // --- Helper Functions for Extraction ---

        fn extract_public_declarations(file_path: &Path) -> anyhow::Result<Vec<PublicDeclaration>> {
            let content = fs::read_to_string(file_path)?;
            let syntax = syn::parse_file(&content).context(format!("Failed to parse file: {:?}", file_path))?;
            
            let mut public_decls = Vec::new();

            for item in syntax.items {
                let mut doc_comments = Vec::new();
                for attr in &item.attrs {
                    if attr.path().is_ident("doc") {
                        if let syn::Meta::NameValue(nv) = &attr.meta {
                            if let syn::Expr::Lit(lit) = &nv.value {
                                if let syn::Lit::Str(s) = &lit.lit {
                                    doc_comments.push(s.value());
                                }
                            }
                        }
                    }
                }
                let documentation = if doc_comments.is_empty() { None } else { Some(doc_comments.join("\n")) };

                match item {
                    syn::Item::Fn(item_fn) => {
                        if matches!(item_fn.vis, syn::Visibility::Public(_)) {
                            let mut qualifiers = Vec::new();
                            if item_fn.sig.asyncness.is_some() { qualifiers.push("async".to_string()); }
                            if item_fn.sig.constness.is_some() { qualifiers.push("const".to_string()); }
                            if item_fn.sig.unsafety.is_some() { qualifiers.push("unsafe".to_string()); }

                            let parameters: Vec<String> = item_fn.sig.inputs.iter().map(|arg| quote!{#arg}.to_string()).collect();
                            let return_type = item_fn.sig.output.as_ref().map(|output| quote!{#output}.to_string());

                            public_decls.push(PublicDeclaration {
                                name: item_fn.sig.ident.to_string(),
                                kind: "fn".to_string(),
                                signature: quote! { #item_fn.sig }.to_string(),
                                documentation,
                                function_details: Some(FunctionDetails {
                                    parameters,
                                    return_type,
                                    qualifiers,
                                    body_snippet: item_fn.block.map(|block| quote! { #block }.to_string()),
                                }),
                                struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Struct(item_struct) => {
                        if matches!(item_struct.vis, syn::Visibility::Public(_)) {
                            let mut fields = HashMap::new();
                            if let syn::Fields::Named(fields_named) = item_struct.fields {
                                for field in fields_named.named {
                                    if let Some(ident) = field.ident {
                                        fields.insert(ident.to_string(), quote!{#field.ty}.to_string());
                                    }
                                }
                            }
                            public_decls.push(PublicDeclaration {
                                name: item_struct.ident.to_string(),
                                kind: "struct".to_string(),
                                signature: quote! { #item_struct.vis #item_struct.ident #item_struct.generics }.to_string(),
                                documentation,
                                struct_details: Some(StructDetails {
                                    fields,
                                    generics: item_struct.generics.to_tokens_string(),
                                }),
                                function_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Enum(item_enum) => {
                        if matches!(item_enum.vis, syn::Visibility::Public(_)) {
                            let variants: Vec<String> = item_enum.variants.iter().map(|v| v.ident.to_string()).collect();
                            public_decls.push(PublicDeclaration {
                                name: item_enum.ident.to_string(),
                                kind: "enum".to_string(),
                                signature: quote! { #item_enum.vis #item_enum.ident #item_enum.generics }.to_string(),
                                documentation,
                                enum_details: Some(EnumDetails {
                                    variants,
                                    generics: item_enum.generics.to_tokens_string(),
                                }),
                                function_details: None, struct_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Const(item_const) => {
                        if matches!(item_const.vis, syn::Visibility::Public(_)) {
                            public_decls.push(PublicDeclaration {
                                name: item_const.ident.to_string(),
                                kind: "const".to_string(),
                                signature: quote! { #item_const.vis #item_const.expr }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Static(item_static) => {
                        if matches!(item_static.vis, syn::Visibility::Public(_)) {
                            public_decls.push(PublicDeclaration {
                                name: item_static.ident.to_string(),
                                kind: "static".to_string(),
                                signature: quote! { #item_static.vis #item_static.ty }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Trait(item_trait) => {
                        if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                            let methods: Vec<FunctionDetails> = item_trait.items.iter().filter_map(|trait_item| {
                                if let syn::TraitItem::Fn(method) = trait_item {
                                    let parameters: Vec<String> = method.sig.inputs.iter().map(|arg| quote!{#arg}.to_string()).collect();
                                    let return_type = method.sig.output.as_ref().map(|output| quote!{#output}.to_string());
                                    let mut qualifiers = Vec::new();
                                    if method.sig.asyncness.is_some() { qualifiers.push("async".to_string()); }
                                    if method.sig.constness.is_some() { qualifiers.push("const".to_string()); }
                                    if method.sig.unsafety.is_some() { qualifiers.push("unsafe".to_string()); }
                                    Some(FunctionDetails {
                                        parameters,
                                        return_type,
                                        qualifiers,
                                        body_snippet: None, // Trait methods often don't have bodies
                                    })
                                } else {
                                    None
                                }
                            }).collect();
                            let associated_types: Vec<String> = item_trait.items.iter().filter_map(|trait_item| {
                                if let syn::TraitItem::Type(ty) = trait_item {
                                    Some(ty.ident.to_string())
                                } else {
                                    None
                                }
                            }).collect();
                            let supertraits: Vec<String> = item_trait.supertraits.iter().map(|st| quote!{#st}.to_string()).collect();

                            public_decls.push(PublicDeclaration {
                                name: item_trait.ident.to_string(),
                                kind: "trait".to_string(),
                                signature: quote! { #item_trait.vis #item_trait.ident #item_trait.generics }.to_string(),
                                documentation,
                                trait_details: Some(TraitDetails {
                                    methods,
                                    associated_types,
                                    generics: item_trait.generics.to_tokens_string(),
                                    supertraits,
                                }),
                                function_details: None, struct_details: None, enum_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Mod(item_mod) => {
                        if matches!(item_mod.vis, syn::Visibility::Public(_)) {
                            public_decls.push(PublicDeclaration {
                                name: item_mod.ident.to_string(),
                                kind: "mod".to_string(),
                                signature: quote! { #item_mod.vis #item_mod.ident }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Type(item_type) => {
                        if matches!(item_type.vis, syn::Visibility::Public(_)) {
                            public_decls.push(PublicDeclaration {
                                name: item_type.ident.to_string(),
                                kind: "type".to_string(),
                                signature: quote! { #item_type.vis #item_type.ident #item_type.generics }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Macro(item_macro) => {
                        if matches!(item_macro.vis, syn::Visibility::Public(_)) {
                            // Difficult to determine exact kind (macro_rules!, proc_macro) from syn::ItemMacro directly
                            // without further parsing or context. Assume macro_rules! for simplicity here.
                            public_decls.push(PublicDeclaration {
                                name: item_macro.mac.path.segments.last().map_or("unknown_macro".to_string(), |s| s.ident.to_string()),
                                kind: "macro".to_string(),
                                signature: quote! { #item_macro }.to_string(), // Full macro invocation is the signature
                                documentation,
                                macro_details: Some(MacroDetails {
                                    kind: "macro_rules".to_string(), // Placeholder, needs smarter detection
                                    definition_snippet: quote! { #item_macro }.to_string(),
                                }),
                                function_details: None, struct_details: None, enum_details: None, trait_details: None,
                            });
                        }
                    }
                    _ => { /* Ignore other item types for now */ }
                }
            }
            Ok(public_decls)
        }

        // Helper for parsing Cargo.toml
        #[derive(Deserialize)]
        struct CargoToml {
            package: Option<PackageSection>,
            dependencies: Option<HashMap<String, toml::Value>>,
        }

        #[derive(Deserialize)]
        struct PackageSection {
            name: String,
        }

        fn extract_dependencies(crate_root: &Path) -> anyhow::Result<HashMap<String, String>> {
            let cargo_toml_path = crate_root.join("Cargo.toml");
            if !cargo_toml_path.exists() {
                return Ok(HashMap::new());
            }
            let content = fs::read_to_string(&cargo_toml_path)?;
            let cargo_toml: CargoToml = toml::from_str(&content)?;

            let mut deps = HashMap::new();
            if let Some(dependencies) = cargo_toml.dependencies {
                for (name, value) in dependencies {
                    if let Some(version) = value.as_str() {
                        deps.insert(name, version.to_string());
                    } else if let Some(table) = value.as_table() {
                        if let Some(version_val) = table.get("version") {
                            if let Some(version_str) = version_val.as_str() {
                                deps.insert(name, version_str.to_string());
                            }
                        }
                    }
                }
            }
            Ok(deps)
        }
        
        // --- Main Analysis Program ---

        fn main() -> anyhow::Result<()> {
            let args: Vec<String> = std::env::args().collect();
            if args.len() < 2 {
                eprintln!("Usage: {} <target_directory> [compiler_version]", args[0]);
                anyhow::bail!("Missing target directory argument");
            }
            let target_dir_str = &args[1];
            let compiler_version = args.get(2).map_or("unknown".to_string(), |s| s.clone());
            let target_dir = PathBuf::from(target_dir_str);

            if !target_dir.exists() {
                anyhow::bail!("Target directory does not exist: {}", target_dir_str);
            }
            if !target_dir.is_dir() {
                anyhow::bail!("Target path is not a directory: {}", target_dir_str);
            }

            let output_dir = PathBuf::from(#output_dir);
            fs::create_dir_all(&output_dir).context(format!("Failed to create output directory: {:?}", output_dir))?;

            let mut all_crate_reports = Vec::new();
            
            // Heuristically find crate roots by looking for Cargo.toml
            for entry in walkdir::WalkDir::new(&target_dir) {
                let entry = entry?;
                if entry.file_type().is_file() && entry.file_name() == "Cargo.toml" {
                    let crate_root = entry.path().parent().context("Cargo.toml has no parent directory")?;
                    let crate_name = crate_root.file_name().map_or("unknown_crate", |s| s.to_string_lossy().as_ref()).to_string();
                    
                    let dependencies = extract_dependencies(crate_root)?;

                    let mut crate_public_decls = Vec::new();
                    // Iterate over all .rs files within this crate's src directory
                    let src_dir = crate_root.join("src");
                    if src_dir.is_dir() {
                        for rs_entry in walkdir::WalkDir::new(&src_dir).max_depth(5) { // Limit depth to avoid too much
                            let rs_entry = rs_entry?;
                            if rs_entry.file_type().is_file() && rs_entry.path().extension().map_or(false, |ext| ext == "rs") {
                                crate_public_decls.extend(extract_public_declarations(rs_entry.path())?);
                            }
                        }
                    }

                    all_crate_reports.push(CrateReport {
                        crate_name,
                        path: crate_root.to_path_buf(),
                        public_decls: crate_public_decls,
                        dependencies,
                    });
                }
            }

            let report = AnalysisReport {
                compiler_version,
                crates: all_crate_reports,
            };

            let serialized = serde_json::to_string_pretty(&report)?;
            // TODO: Implement actual chunking logic based on #chunk_size
            // For now, write a single file.
            let output_file_path = output_dir.join("analysis_report_chunk_0.json");
            fs::write(&output_file_path, serialized)
                .context(format!("Failed to write report to {:?}", output_file_path))?;
            eprintln!("Analysis report generated to {:?}", output_file_path);

            Ok(())
        }
    };
    generated_code.into()
}

// Helper trait to convert any type that can be quoted into a string
trait ToTokensString {
    fn to_tokens_string(&self) -> Option<String>;
}

impl<T: quote::ToTokens> ToTokensString for T {
    fn to_tokens_string(&self) -> Option<String> {
        let tokens = quote! { #self };
        if tokens.is_empty() {
            None
        } else {
            Some(tokens.to_string())
        }
    }
}