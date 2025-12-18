use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as SynResult},
    parse_macro_input,
    Attribute,
    Ident,
    Item,
    ItemConst,
    ItemEnum,
    ItemFn,
    ItemMacro,
    ItemMod,
    ItemStatic,
    ItemStruct,
    ItemTrait,
    ItemType,
    LitBool,
    LitInt,
    LitStr,
    Token,
    Type,
    Visibility,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir; // Added dependency
use anyhow::Context; // Added dependency

mod grast_macro; // Declare the new module
mod autoheader; // Declare the new module

pub use grast_macro::*; // Re-export the grast macro
pub use autoheader::*; // Re-export autoheader

// --- mkslop_macro (moved from mkslop-macros/src/mkslop_macro.rs) ---

/// A procedural macro, currently acting as an identity macro for string literals.
///
/// This macro was originally intended for applying auto-fixes to AI-generated code
/// format string issues, but its core logic (`fix_cfg_format_string`) is
/// currently unresolved. For now, it simply returns its string literal input.
#[proc_macro]
pub fn mkslop(input: TokenStream) -> TokenStream {
    // Parse the input as a single LitStr
    let input_lit = parse_macro_input!(input as LitStr);

    // For now, act as an identity macro for LitStr
    input_lit.to_token_stream().into()
}

// --- lru_cache (moved from lru_cache.rs) ---

// Helper function that was previously in lru_cache.rs
// It implements the logic for the `lru` attribute macro.
fn lru_macro_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return syn::Error::new_spanned(args.into_iter().collect::<proc_macro2::TokenStream>(), "The `lru` macro does not accept arguments.").to_compile_error().into();
    }

    let item = parse_macro_input!(input as ItemFn);

    // Get the name of the function
    let fn_name = &item.sig.ident;
    let cache_name = Ident::new(&format!("__LRU_CACHE_{}", fn_name), fn_name.span());

    // Generate code for the LRU cache
    let expanded = quote! {
        // Define a static LRU cache for this function
        static #cache_name: once_cell::sync::Lazy<
            std::sync::Mutex<lru::LruCache<
                std::collections::hash_map::DefaultHasher, // Key (hashed arguments)
                // Need to specify the return type of the function for the value
                // For a real implementation, you'd need to parse the function's return type
                // For now, let's assume a simple type or require user to specify.
                // This is a placeholder and will likely require more advanced syn parsing
                // of the function signature.
                // For demonstration, let's use a dummy type, or just return the original item.
                String // Placeholder: should be the function's return type
            >>
        > = once_cell::sync::Lazy::new(|| {
            std::sync::Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(100).unwrap()))
        });

        // Original function definition, potentially wrapped
        #item
    };
    expanded.into()
}


/// The `lru` attribute macro.
///
/// This macro acts as a shim to correctly route the attribute macro invocation
/// to its implementation within the `lru_cache` module, satisfying the
/// proc-macro crate limitations that require `#[proc_macro_attribute]` functions
/// to reside in the root of the crate.
#[proc_macro_attribute]
pub fn lru(args: TokenStream, input: TokenStream) -> TokenStream {
    lru_macro_impl(args, input)
}


// --- wrap_pub (moved from wrap_pub.rs) ---

#[proc_macro_attribute]
pub fn wrap_pub_items(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr), // Convert to proc_macro2::TokenStream for new_spanned
            "#[wrap_pub_items] currently does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }

    let mut input_module = parse_macro_input!(item as ItemMod);

    if let Some((_, items)) = &mut input_module.content {
        for item in items.iter_mut() {
            let is_public = match item {
                Item::Const(item_const) => matches!(item_const.vis, Visibility::Public(_)),
                Item::Enum(item_enum) => matches!(item_enum.vis, Visibility::Public(_)),
                Item::Fn(item_fn) => matches!(item_fn.vis, Visibility::Public(_)),
                Item::Mod(item_mod) => matches!(item_mod.vis, Visibility::Public(_)),
                Item::Static(item_static) => matches!(item_static.vis, Visibility::Public(_)),
                Item::Struct(item_struct) => matches!(item_struct.vis, Visibility::Public(_)),
                Item::Trait(item_trait) => matches!(item_trait.vis, Visibility::Public(_)),
                Item::Type(item_type) => matches!(item_type.vis, Visibility::Public(_)),
                _ => false, // Other items are not typically 'pub' in the same way, or can't be wrapped easily.
            };

            if is_public {
                // Prepend a new attribute macro to the public item
                // This assumes `__my_public_item_wrapper_attr` will be defined elsewhere as an attribute macro.
                let new_attr: Attribute = syn::parse_quote! { #[__my_public_item_wrapper_attr] };

                match item {
                    Item::Const(item_const) => item_const.attrs.insert(0, new_attr),
                    Item::Enum(item_enum) => item_enum.attrs.insert(0, new_attr),
                    Item::Fn(item_fn) => item_fn.attrs.insert(0, new_attr),
                    Item::Mod(item_mod) => item_mod.attrs.insert(0, new_attr),
                    Item::Static(item_static) => item_static.attrs.insert(0, new_attr),
                    Item::Struct(item_struct) => item_struct.attrs.insert(0, new_attr),
                    Item::Trait(item_trait) => item_trait.attrs.insert(0, new_attr),
                    Item::Type(item_type) => item_type.attrs.insert(0, new_attr),
                    _ => {}, // Should not happen given the 'is_public' check, but good for completeness.
                }
            }
        }
    }

    quote! { #input_module }.into()
}


// --- public_item_wrapper (moved from public_item_wrapper.rs) ---

/// A placeholder attribute macro that will be applied to public items
/// by `#[wrap_pub_items]`.
///
/// This macro currently just prints the item it's attached to and returns it unmodified.
/// You would implement your desired wrapping logic here.
#[proc_macro_attribute]
pub fn __my_public_item_wrapper_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For demonstration, let's just print the item.
    // In a real scenario, you would parse 'item' using syn,
    // apply your wrapping logic, and then use quote! to generate the new TokenStream.
    let parsed_item = parse_macro_input!(item as Item);
    eprintln!("__my_public_item_wrapper_attr applied to item: {:?}", parsed_item);

    // Return the item unmodified for now.
    quote! { #parsed_item }.into()
}


// --- nix_rustc_analysis (moved from nix_rustc_analysis.rs) ---

/// A function-like procedural macro that attempts to analyze Rustc source code.
/// It takes a string literal representing the path to the Rustc source directory.
///
/// Usage:
/// trynixrustc!("/path/to/rustc/source");
#[proc_macro]
pub fn trynixrustc(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as LitStr);
    let rustc_source_path_str = path_lit.value();
    let rustc_source_path = PathBuf::from(rustc_source_path_str);

    if !rustc_source_path.exists() {
        return syn::Error::new_spanned(
            path_lit, // Changed from path_lit.span() to path_lit directly as LitStr implements ToTokens
            format!("Rustc source path does not exist: {}", rustc_source_path_str),
        )
        .to_compile_error()
        .into();
    }
    if !rustc_source_path.is_dir() {
        return syn::Error::new_spanned(
            path_lit, // Changed from path_lit.span() to path_lit directly as LitStr implements ToTokens
            format!("Rustc source path is not a directory: {}", rustc_source_path_str),
        )
        .to_compile_error()
        .into();
    }

    let mut found_crate_paths = Vec::new();

    for entry in WalkDir::new(&rustc_source_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if path.ends_with("src/lib.rs") || path.ends_with("src/main.rs") {
                // We found a potential crate root (or binary root)
                // The parent's parent should be the crate directory
                if let Some(crate_dir) = path.parent().and_then(|p| p.parent()) {
                    found_crate_paths.push(crate_dir.to_path_buf());
                }
            }
        }
    }

    let mut output = quote! {};
    if found_crate_paths.is_empty() {
        output = quote! {
            compile_error! { "No src/lib.rs or src/main.rs found in the provided path." }
        };
    } else {
        let paths_str = found_crate_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n- ");
        let msg = format!("Found the following crate directories:\n- {}", paths_str);
        output = quote! {
            // Use eprintln! to print during macro expansion, not compile_error!
            // or use a custom 'info!' macro if available in the project.
            eprintln!(#msg);
        };
    }

    output.into()
}


// --- code_search (moved from code_search.rs) ---

struct FindCodeArgs {
    pattern: LitStr,
    file_types: Option<Vec<LitStr>>,
    case_sensitive: Option<LitBool>,
    context_lines: Option<LitInt>,
    dir: Option<LitStr>,
}

impl Parse for FindCodeArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut pattern = None;
        let mut file_types = None;
        let mut case_sensitive = None;
        let mut context_lines = None;
        let mut dir = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            if key == "pattern" {
                pattern = Some(input.parse()?);
            } else if key == "file_types" {
                let content;
                syn::bracketed!(content in input);
                let types_parsed: syn::punctuated::Punctuated<LitStr, Token![,]> =
                    content.parse_terminated(|input| LitStr::parse(input), Token![,])?; // Fix E0308
                file_types = Some(types_parsed.into_iter().collect());
            } else if key == "case_sensitive" {
                case_sensitive = Some(input.parse()?);
            } else if key == "context_lines" {
                context_lines = Some(input.parse()?);
            } else if key == "dir" {
                dir = Some(input.parse()?);
            } else {
                return Err(input.error(format!("unexpected argument: {}", key)));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(FindCodeArgs {
            pattern: pattern.ok_or_else(|| input.error("missing 'pattern' argument"))?,
            file_types,
            case_sensitive,
            context_lines,
            dir,
        })
    }
}

#[proc_macro]
pub fn findcode(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as FindCodeArgs);

    let mut command_parts = vec!["grep -R".to_string()];

    // Case sensitivity
    if !args.case_sensitive.map_or(false, |l| l.value()) {
        command_parts.push("-i".to_string());
    }

    // Context lines
    if let Some(lines) = args.context_lines {
        command_parts.push(format!("-C {}", lines.base10_parse::<u32>().unwrap()));
    }

    // File types (include)
    if let Some(types) = args.file_types {
        for ft in types {
            command_parts.push(format!("--include='*.{{}}'", ft.value()));
        }
    }

    // Pattern
    command_parts.push(format!("-- '{}'", args.pattern.value()));

    // Directory
    let search_dir = args.dir.map_or(".".to_string(), |d| d.value());
    command_parts.push(search_dir);

    let grep_command = command_parts.join(" ");

    // For now, expand to a println! that shows the grep command.
    // In a more advanced scenario, this might be executed by a helper.
    let output = quote! {
        // You would typically execute this command in a shell, not directly in Rust code.
        // This macro just expands to a print statement that gives you the command to run.
        eprintln!("Generated grep command: {}", #grep_command);
    };

    output.into()
}


// --- macroslop_driver (moved from macroslop_driver.rs) ---

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

// --- Data Structures for Report (copied from macroslop_driver.rs) ---

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FunctionDetails {
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub qualifiers: Vec<String>, // e.g., "async", "const", "unsafe"
    pub body_snippet: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct StructDetails {
    pub fields: HashMap<String, String>, // field_name -> field_type
    pub generics: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EnumDetails {
    pub variants: Vec<String>,
    pub generics: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TraitDetails {
    pub methods: Vec<FunctionDetails>, // Only method signatures
    pub associated_types: Vec<String>,
    pub generics: Option<String>,
    pub supertraits: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MacroDetails {
    pub kind: String, // e.g., "macro_rules", "proc_macro", "proc_macro_attribute", "proc_macro_derive"
    pub definition_snippet: String, // The full token stream of the macro definition
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CrateReport {
    pub crate_name: String,
    pub path: PathBuf,
    pub public_decls: Vec<PublicDeclaration>,
    pub dependencies: HashMap<String, String>, // name -> version
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AnalysisReport {
    pub compiler_version: String,
    pub crates: Vec<CrateReport>,
}

// --- Helper Functions for Extraction (copied from macroslop_driver.rs) ---

fn extract_public_declarations(file_path: &Path) -> anyhow::Result<Vec<PublicDeclaration>> {
    let content = std::fs::read_to_string(file_path)?;
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

                    // Fix E0425: `arg` and `output` are not directly in scope of `quote!{}` closure
                    let parameters: Vec<String> = item_fn.sig.inputs.iter().map(|farg| quote!{#farg}.to_string()).collect();
                    let return_type = item_fn.sig.output.as_ref().map(|foutput| quote!{#foutput}.to_string());

                    let sig_tokens = &item_fn.sig; // Capture for quote!
                    let block_tokens = item_fn.block.as_ref().map(|b| quote!{#b}.to_string()); // Capture for quote!

                    public_decls.push(PublicDeclaration {
                        name: item_fn.sig.ident.to_string(),
                        kind: "fn".to_string(),
                        signature: quote! { #sig_tokens }.to_string(), // Fixed E0425
                        documentation,
                        function_details: Some(FunctionDetails {
                            parameters,
                            return_type,
                            qualifiers,
                            body_snippet: block_tokens, // Fixed E0425
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
                            // Fix E0425: `field` is not directly in scope of `quote!{}` closure
                            let field_ty = &field.ty;
                            if let Some(ident) = field.ident {
                                fields.insert(ident.to_string(), quote!{#field_ty}.to_string());
                            }
                        }
                    }
                    let struct_tokens = &item_struct; // Capture for quote!
                    let generics_tokens = item_struct.generics.to_tokens_string(); // Already correct

                    public_decls.push(PublicDeclaration {
                        name: item_struct.ident.to_string(),
                        kind: "struct".to_string(),
                        signature: quote! { #struct_tokens.vis #struct_tokens.ident #struct_tokens.generics }.to_string(), // Fixed E0425
                        documentation,
                        struct_details: Some(StructDetails {
                            fields,
                            generics: generics_tokens,
                        }),
                        function_details: None, enum_details: None, trait_details: None, macro_details: None,
                    });
                }
            },
            syn::Item::Enum(item_enum) => {
                if matches!(item_enum.vis, syn::Visibility::Public(_)) {
                    let variants: Vec<String> = item_enum.variants.iter().map(|v| v.ident.to_string()).collect();
                    let enum_tokens = &item_enum; // Capture for quote!
                    let generics_tokens = item_enum.generics.to_tokens_string(); // Already correct

                    public_decls.push(PublicDeclaration {
                        name: item_enum.ident.to_string(),
                        kind: "enum".to_string(),
                        signature: quote! { #enum_tokens.vis #enum_tokens.ident #enum_tokens.generics }.to_string(), // Fixed E0425
                        documentation,
                        enum_details: Some(EnumDetails {
                            variants,
                            generics: generics_tokens,
                        }),
                        function_details: None, struct_details: None, trait_details: None, macro_details: None,
                    });
                }
            },
            syn::Item::Const(item_const) => {
                if matches!(item_const.vis, syn::Visibility::Public(_)) {
                    let const_tokens = &item_const; // Capture for quote!
                    public_decls.push(PublicDeclaration {
                        name: item_const.ident.to_string(),
                        kind: "const".to_string(),
                        signature: quote! { #const_tokens.vis #const_tokens.expr }.to_string(), // Fixed E0425
                        documentation,
                        function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                    });
                }
            },
            syn::Item::Static(item_static) => {
                if matches!(item_static.vis, syn::Visibility::Public(_)) {
                    let static_tokens = &item_static; // Capture for quote!
                    public_decls.push(PublicDeclaration {
                        name: item_static.ident.to_string(),
                        kind: "static".to_string(),
                        signature: quote! { #static_tokens.vis #static_tokens.ty }.to_string(), // Fixed E0425
                        documentation,
                        function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                    });
                }
            },
            syn::Item::Trait(item_trait) => {
                if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    let methods: Vec<FunctionDetails> = item_trait.items.iter().filter_map(|trait_item| {
                        if let syn::TraitItem::Fn(method) = trait_item {
                            // Fix E0425: `arg` and `output`
                            let parameters: Vec<String> = method.sig.inputs.iter().map(|farg| quote!{#farg}.to_string()).collect();
                            let return_type = method.sig.output.as_ref().map(|foutput| quote!{#foutput}.to_string());
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
                    // Fix E0425: `st`
                    let supertraits: Vec<String> = item_trait.supertraits.iter().map(|fst| quote!{#fst}.to_string()).collect();
                    let trait_tokens = &item_trait;
                    let generics_tokens = item_trait.generics.to_tokens_string();

                    public_decls.push(PublicDeclaration {
                        name: item_trait.ident.to_string(),
                        kind: "trait".to_string(),
                        signature: quote! { #trait_tokens.vis #trait_tokens.ident #trait_tokens.generics }.to_string(),
                        documentation,
                        trait_details: Some(TraitDetails {
                            methods,
                            associated_types,
                            generics: generics_tokens,
                            supertraits,
                        }),
                        function_details: None, struct_details: None, enum_details: None, macro_details: None,
                    });
                }
            },
            syn::Item::Mod(item_mod) => {
                if matches!(item_mod.vis, syn::Visibility::Public(_)) {
                    let mod_tokens = &item_mod;
                    public_decls.push(PublicDeclaration {
                                name: item_mod.ident.to_string(),
                                kind: "mod".to_string(),
                                signature: quote! { #mod_tokens.vis #mod_tokens.ident }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Type(item_type) => {
                        if matches!(item_type.vis, syn::Visibility::Public(_)) {
                            let type_tokens = &item_type;
                            let generics_tokens = item_type.generics.to_tokens_string();
                            public_decls.push(PublicDeclaration {
                                name: item_type.ident.to_string(),
                                kind: "type".to_string(),
                                signature: quote! { #type_tokens.vis #type_tokens.ident #type_tokens.generics }.to_string(),
                                documentation,
                                function_details: None, struct_details: None, enum_details: None, trait_details: None, macro_details: None,
                            });
                        }
                    },
                    syn::Item::Macro(item_macro) => {
                        if matches!(item_macro.vis, syn::Visibility::Public(_)) {
                            let macro_tokens = &item_macro;
                            public_decls.push(PublicDeclaration {
                                name: item_macro.mac.path.segments.last().map_or("unknown_macro".to_string(), |s| s.ident.to_string()),
                                kind: "macro".to_string(),
                                signature: quote! { #macro_tokens }.to_string(),
                                documentation,
                                macro_details: Some(MacroDetails {
                                    kind: "macro_rules".to_string(), // Placeholder, needs smarter detection
                                    definition_snippet: quote! { #macro_tokens }.to_string(),
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

        fn extract_dependencies_inner(crate_root: &Path) -> anyhow::Result<HashMap<String, String>> {
            let cargo_toml_path = crate_root.join("Cargo.toml");
            if !cargo_toml_path.exists() {
                return Ok(HashMap::new());
            }
            let content = std::fs::read_to_string(&cargo_toml_path)?;
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

// --- grast (moved from grast.rs) ---

use grast_core::GrastDb; // Import GrastDb from our new core library

/// A procedural macro that converts Rust code into RDF Turtle representation
/// using the `grast_core` library.
///
/// This macro expands to a `&'static str` literal containing the Turtle representation.
///
/// Usage:
/// ```ignore
/// let turtle_output: &str = grast! {
///     pub fn my_function(arg1: u32, arg2: String) -> Vec<String> {
///         println!("Hello, {}!", arg1);
///         let result = vec![arg2];
///         result
///     }
/// };
/// ```
#[proc_macro]
pub fn grast(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a syn::File
    let ast_file = parse_macro_input!(input as syn::File);

    // Create a GrastDb and flatten the AST into it
    let mut db = GrastDb::new();
    db.flatten(&ast_file);

    // Convert the GrastDb to its Turtle string representation
    let turtle_string = db.to_turtle();

    // The macro should expand to a string literal containing the turtle_string
    // We escape the string to ensure it's a valid Rust string literal.
    let expanded = quote! {
        #turtle_string
    };

    expanded.into()
}
