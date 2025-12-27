use anyhow::{Context, Result};
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write; // Only Write is used
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::{
    visit_mut::{visit_item_fn_mut, VisitMut},
    File, // This is syn::File
    Item,
    ItemFn,
    ItemUse,
};
use toml;
use proc_macro2;
use toml_edit::{self, Document, Table}; // Import Document and Table from toml_edit

/// --- Configuration Structures for Edit Jobs ---

#[derive(Debug, Deserialize)]
#[decl(struct, name = "EditJobConfig", vis = "pub", hash = "54d41fac")]
pub struct EditJobConfig {
    pub edits: Vec<EditJob>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[decl(enum, name = "EditJob", vis = "pub", hash = "74f33c7a")]
pub enum EditJob {
    AddUse(AddUseDetails),
    RemoveFunction(RemoveFunctionDetails),
    ReplaceExpression(ReplaceExpressionDetails),
    AddFunction(AddFunctionDetails),
    AddItem(AddItemDetails),
    ReplaceFileContent(ReplaceFileContentDetails),
    ReplaceFileContentFromFile(ReplaceFileContentFromFileDetails),
    RunSearch(RunSearchDetails), // New variant for running search commands
    RemoveCargoDependency(RemoveCargoDependencyDetails), // New variant for removing Cargo dependencies
    RemoveUse(RemoveUseDetails), // New variant for removing use statements
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "AddUseDetails", vis = "pub", hash = "5da0b580")]
pub struct AddUseDetails {
    pub target_file: PathBuf,
    pub path: String, // e.g., "use super::error::AppError;"
    pub position: Option<AddUsePosition>,
}

#[derive(Debug, Deserialize)]
#[decl(enum, name = "AddUsePosition", vis = "pub", hash = "d336341d")]
pub enum AddUsePosition {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "after_use_path")]
    AfterUsePath(String), // Path of an existing use statement to place after
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "RemoveFunctionDetails", vis = "pub", hash = "f102264d")]
pub struct RemoveFunctionDetails {
    pub target_file: PathBuf,
    pub function_name: String,
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "ReplaceExpressionDetails", vis = "pub", hash = "bed16452")]
pub struct ReplaceExpressionDetails {
    pub target_file: PathBuf,
    pub function_name: String, // Function containing the expression
    pub old_code_snippet: String, // The snippet to replace
    pub new_code_snippet: String, // The replacement snippet
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "AddFunctionDetails", vis = "pub", hash = "91833729")]
pub struct AddFunctionDetails {
    pub target_file: PathBuf,
    pub function_code: String, // Full code of the function to add
}

// New struct for adding any Item (struct, enum, function, impl, etc.)
#[derive(Debug, Deserialize)]
#[decl(struct, name = "AddItemDetails", vis = "pub", hash = "e1e7721e")]
pub struct AddItemDetails {
    pub target_file: PathBuf,
    pub item_code: String, // Full code of the item to add
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "ReplaceFileContentDetails", vis = "pub", hash = "526a6d16")]
pub struct ReplaceFileContentDetails {
    pub target_file: PathBuf,
    pub new_content: String,
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "ReplaceFileContentFromFileDetails", vis = "pub", hash = "6af2c75b")]
pub struct ReplaceFileContentFromFileDetails {
    pub target_file: PathBuf,
    pub source_file: PathBuf,
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "RunSearchDetails", vis = "pub", hash = "adc7dbbd")]
pub struct RunSearchDetails {
    pub command: String, // The shell command to execute, e.g., "rg rocksdb Cargo.toml"
    pub output_file: Option<PathBuf>, // Optional file to save the output to
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "RemoveCargoDependencyDetails", vis = "pub", hash = "10f3b1ee")]
pub struct RemoveCargoDependencyDetails {
    pub target_file: PathBuf, // Path to the Cargo.toml file
    pub package_name: String, // Name of the dependency package to remove or member path from workspace.members
}

#[derive(Debug, Deserialize)]
#[decl(struct, name = "RemoveUseDetails", vis = "pub", hash = "5d5036bd")]
pub struct RemoveUseDetails {
    pub target_file: PathBuf,
    pub use_path: String, // The path inside the use statement, e.g., "rocksdb::{Options, DB}"
}

// --- Helper Functions ---

/// Helper function to get a list of config files.
/// If path is a file, returns a vector containing just that path.
/// If path is a directory, returns all .toml files within it, sorted by name.
fn get_config_paths(path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if path.is_file() {
        paths.push(path.to_path_buf());
    } else if path.is_dir() {
        let mut dir_entries: Vec<_> = fs::read_dir(path)?.filter_map(|entry| {
            let entry = entry.ok()?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "toml") {
                Some(entry_path)
            } else {
                None
            }
        }).collect();
        dir_entries.sort(); // Sort by name
        paths.extend(dir_entries);
    } else {
        anyhow::bail!("Path {:?} is neither a file nor a directory.", path);
    }
    Ok(paths)
}

/// --- Edit Application Functions ---

fn apply_add_use(ast: &mut syn::File, details: &AddUseDetails) -> Result<()> {
    let new_use_item: ItemUse = syn::parse_str(&details.path)
        .with_context(|| format!("Invalid use statement syntax: {}", details.path))?;

    let new_use_str = quote! { #new_use_item }.to_string();

    // Check for duplicates
    for item in &ast.items {
        if let Item::Use(existing_use) = item {
            if quote! { #existing_use }.to_string() == new_use_str {
                eprintln!("  Skipping duplicate use statement: {}", details.path);
                return Ok(())
            }
        }
    }

    match &details.position {
        Some(AddUsePosition::Start) => {
            eprintln!("  Inserting use statement at start.");
            ast.items.insert(0, Item::Use(new_use_item.clone()));
        }
        Some(AddUsePosition::End) | None => {
            eprintln!("  Inserting use statement at end.");
            // Find the last use statement and insert after it
            let mut last_use_idx = 0;
            for (idx, item) in ast.items.iter().enumerate() {
                if let Item::Use(_) = item {
                    last_use_idx = idx;
                }
            }
            ast.items.insert(last_use_idx + 1, Item::Use(new_use_item.clone()));
        }
        Some(AddUsePosition::AfterUsePath(after_path)) => {
            eprintln!("  Inserting use statement after path: {}", after_path);
            let after_use_item: ItemUse = syn::parse_str(after_path)
                .with_context(|| format!("Invalid 'after_use_path' syntax: {}", after_path))?;
            let after_use_str = quote! { #after_use_item }.to_string();

            let mut inserted = false;
            for (idx, item) in ast.items.iter().enumerate() {
                if let Item::Use(existing_use) = item {
                    if quote! { #existing_use }.to_string() == after_use_str {
                        ast.items.insert(idx + 1, Item::Use(new_use_item.clone()));
                        inserted = true;
                        break;
                    }
                }
            }
            if !inserted {
                eprintln!("  Warning: 'after_use_path' not found. Inserting at end of uses.");
                ast.items.push(Item::Use(new_use_item));
            }
        }
    }
    eprintln!("  Added use statement: {}", details.path);
    Ok(())
}

fn apply_remove_use(ast: &mut syn::File, details: &RemoveUseDetails) -> Result<()> {
    let use_to_remove: ItemUse = syn::parse_str(&format!("use {};", details.use_path))
        .with_context(|| format!("Invalid use statement path: {}", details.use_path))?;

    let use_to_remove_str = quote! { #use_to_remove }.to_string();
    let mut removed = false;

    ast.items.retain(|item| {
        if let Item::Use(existing_use) = item {
            if quote! { #existing_use }.to_string() == use_to_remove_str {
                removed = true;
                return false; // Remove this use statement
            }
        }
        true // Keep other items
    });

    if removed {
        eprintln!("  Removed use statement: {}", details.use_path);
    } else {
        eprintln!("  Warning: Use statement '{}' not found for removal.", details.use_path);
    }
    Ok(())
}


fn apply_remove_function(ast: &mut syn::File, details: &RemoveFunctionDetails) -> Result<()> {
    let mut removed = false;
    ast.items.retain(|item| {
        if let Item::Fn(item_fn) = item {
            if item_fn.sig.ident == details.function_name {
                removed = true;
                return false; // Remove this function
            }
        }
        true // Keep other items
    });
    if removed {
        eprintln!("  Removed function: {}", details.function_name);
    } else {
        eprintln!("  Warning: Function '{}' not found for removal.", details.function_name);
    }
    Ok(())
}

fn apply_replace_expression(ast: &mut syn::File, details: &ReplaceExpressionDetails) -> Result<()> {
    let mut visitor = ExpressionReplacer {
        function_name: &details.function_name,
        old_snippet: syn::parse_str(&details.old_code_snippet)
            .context("Failed to parse old code snippet")?,
        new_snippet: syn::parse_str(&details.new_code_snippet)
            .context("Failed to parse new code snippet")?,
        replaced_count: 0,
    };
    visitor.visit_file_mut(ast);

    if visitor.replaced_count > 0 {
        eprintln!(
            "  Replaced {} occurrences of '{}' with '{}' in function '{}'.",
            visitor.replaced_count, details.old_code_snippet, details.new_code_snippet, details.function_name
        );
    } else {
        eprintln!(
            "  Warning: No occurrences of '{}' found in function '{}' for replacement.",
            details.old_code_snippet, details.function_name
        );
    }
    Ok(())
}

struct ExpressionReplacer<'a> {
    function_name: &'a str,
    old_snippet: syn::Expr,
    new_snippet: syn::Expr,
    replaced_count: usize,
}

impl<'a> VisitMut for ExpressionReplacer<'a> {
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        if i.sig.ident == self.function_name {
            // Found the target function, now visit its block
            self.visit_block_mut(&mut i.block);
        }
        // Continue visiting other functions if any
        visit_item_fn_mut(self, i); // Call default visitor for ItemFn to go deeper
    }

    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        use quote::ToTokens;
        let mut i_tokens = proc_macro2::TokenStream::new();
        i.to_tokens(&mut i_tokens);
        let mut old_snippet_tokens = proc_macro2::TokenStream::new();
        self.old_snippet.to_tokens(&mut old_snippet_tokens);

        if i_tokens.to_string() == old_snippet_tokens.to_string() {
            *i = self.new_snippet.clone();
            self.replaced_count += 1;
        }
        // Important: Recurse into children of the expression
        syn::visit_mut::visit_expr_mut(self, i);
    }
}

fn apply_add_function(ast: &mut syn::File, details: &AddFunctionDetails) -> Result<()> {
    let new_fn: ItemFn = syn::parse_str(&details.function_code)
        .with_context(|| format!("Invalid function code syntax: {}", details.function_code))?;

    // Check if a function with the same name already exists
    let new_fn_name = new_fn.sig.ident.to_string();
    for item in &ast.items {
        if let Item::Fn(existing_fn) = item {
            if existing_fn.sig.ident.to_string() == new_fn_name {
                eprintln!("  Warning: Function '{}' already exists. Skipping addition.", new_fn_name);
                return Ok(())
            }
        }
    }

    ast.items.push(Item::Fn(new_fn));
    eprintln!("  Added function: {}", new_fn_name);
    Ok(())
}

fn apply_add_item(ast: &mut syn::File, details: &AddItemDetails) -> Result<()> {
    let new_item: Item = syn::parse_str(&details.item_code)
        .with_context(|| format!("Invalid item code syntax: {}", details.item_code))?;

    // Check for duplicates based on item type and name
    let item_name = match &new_item {
        Item::Const(i) => Some(i.ident.to_string()),
        Item::Enum(i) => Some(i.ident.to_string()),
        Item::Fn(i) => Some(i.sig.ident.to_string()),
        Item::Struct(i) => Some(i.ident.to_string()),
        Item::Macro(i) => i.ident.as_ref().map(|id| id.to_string()),
        Item::Mod(i) => Some(i.ident.to_string()),
        Item::Trait(i) => Some(i.ident.to_string()),
        Item::TraitAlias(i) => Some(i.ident.to_string()),
        Item::Type(i) => Some(i.ident.to_string()),
        Item::Union(i) => Some(i.ident.to_string()),
        Item::Use(_i) => None, // Use statements are handled by apply_add_use with different logic
        _ => None, // For other item types, we might not have a simple name or want to check duplicates
    };

    if let Some(name) = item_name {
        for item in &ast.items {
            let existing_name = match item {
                Item::Const(i) => Some(i.ident.to_string()),
                Item::Enum(i) => Some(i.ident.to_string()),
                Item::Fn(i) => Some(i.sig.ident.to_string()),
                Item::Struct(i) => Some(i.ident.to_string()),
                Item::Macro(i) => i.ident.as_ref().map(|id| id.to_string()),
                Item::Mod(i) => Some(i.ident.to_string()),
                Item::Trait(i) => Some(i.ident.to_string()),
                Item::TraitAlias(i) => Some(i.ident.to_string()),
                Item::Type(i) => Some(i.ident.to_string()),
                Item::Union(i) => Some(i.ident.to_string()),
                _ => None,
            };
            if existing_name.map_or(false, |n| n == name) {
                eprintln!("  Warning: Item '{}' already exists. Skipping addition.", name);
                return Ok(());
            }
        }
    }


    ast.items.push(new_item);
    eprintln!("  Added item to file.");
    Ok(())
}

fn apply_replace_file_content(file_path: &PathBuf, new_content: &str) -> Result<()> {
    fs::write(file_path, new_content)
        .with_context(|| format!("Failed to write modified file: {:?}", file_path))?;
    Ok(())
}

fn apply_replace_file_content_from_file(target_file: &PathBuf, source_file: &PathBuf) -> Result<()> {
    let new_content = fs::read_to_string(source_file)
        .with_context(|| format!("Failed to read source file: {:?}", source_file))?;
    fs::write(target_file, new_content)
        .with_context(|| format!("Failed to write modified file: {:?}", target_file))?;
    Ok(())
}

fn apply_run_search(details: &RunSearchDetails) -> Result<()> {
    eprintln!("  Running search command: '{}'", details.command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&details.command)
        .output()
        .with_context(|| format!("Failed to execute command: {}", details.command))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    eprintln!("  Search stdout:\n{}", stdout);
    eprintln!("  Search stderr:\n{}", stderr);

    if let Some(output_file) = &details.output_file {
        let mut file = fs::File::create(output_file)
            .with_context(|| format!("Failed to create output file: {:?}", output_file))?;
        file.write_all(stdout.as_bytes())
            .with_context(|| format!("Failed to write stdout to file: {:?}", output_file))?;
        file.write_all(stderr.as_bytes())
            .with_context(|| format!("Failed to write stderr to file: {:?}", output_file))?;
        eprintln!("  Search output saved to {:?}", output_file);
    }
    Ok(())
}

fn apply_remove_cargo_dependency(details: &RemoveCargoDependencyDetails) -> Result<()> {
    eprintln!(
        "  Removing dependency '{}' from Cargo.toml: {:?}",
        details.package_name, details.target_file
    );

    let content = fs::read_to_string(&details.target_file)
        .with_context(|| format!("Failed to read Cargo.toml: {:?}", details.target_file))?;

    let mut doc = content.parse::<Document<String>>()
        .with_context(|| format!("Failed to parse Cargo.toml: {:?}", details.target_file))?;

    let mut changed = false;

    // Get mutable reference to the root Table of the document
    let root_table = doc.as_table_mut()
        .with_context(|| format!("Cargo.toml root is not a table: {:?}", details.target_file))?;

    // Helper to remove dependency from a section, operating on a &mut Table
    let mut remove_from_table_section = |table: &mut toml_edit::Table, section_key: &str, package_name: &str| {
        if let Some(item) = table.get_mut(section_key) {
            if let Some(sub_table) = item.as_table_mut() {
                if sub_table.contains_key(package_name) {
                    sub_table.remove(package_name);
                    eprintln!("    Removed '{}' from [{}]", package_name, section_key);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    // Apply to direct dependency sections
    changed |= remove_from_table_section(root_table, "dependencies", &details.package_name);
    changed |= remove_from_table_section(root_table, "dev-dependencies", &details.package_name);
    changed |= remove_from_table_section(root_table, "build-dependencies", &details.package_name);
    
    // Handle workspace.dependencies
    // Note: get_mut on Table returns Option<&mut Item>, so we need to check if it's a table
    if let Some(workspace_item) = root_table.get_mut("workspace") {
        if let Some(workspace_table) = workspace_item.as_table_mut() {
            changed |= remove_from_table_section(workspace_table, "dependencies", &details.package_name);
        }
    }

    // Also handle patch section
    if let Some(patch_item) = root_table.get_mut("patch.crates-io") {
        if let Some(patch_table) = patch_item.as_table_mut() {
            changed |= remove_from_table_section(patch_table, "patch.crates-io", &details.package_name);
        }
    }

    // Handle workspace.members array (this needs a slightly different approach)
    if let Some(workspace_item) = root_table.get_mut("workspace") {
        if let Some(workspace_table) = workspace_item.as_table_mut() {
            if let Some(members_item) = workspace_table.get_mut("members") {
                if let Some(members_array) = members_item.as_array_mut() {
                    let initial_len = members_array.len();
                    members_array.retain(|item| {
                        item.as_str().map_or(true, |s| s != details.package_name)
                    });
                    if members_array.len() < initial_len {
                        eprintln!("    Removed '{}' from [workspace.members]", details.package_name);
                        changed = true;
                    }
                }
            }
        }
    }

    if changed {
        // We modify doc.root_table, so doc.to_string() will reflect changes.
        fs::write(&details.target_file, doc.to_string().as_bytes())
            .with_context(|| format!("Failed to write modified Cargo.toml: {:?}", details.target_file))?;
        eprintln!("  Successfully removed dependency '{}' from {:?}", details.package_name, details.target_file);
    } else {
        eprintln!("  Dependency '{}' not found in any relevant section of {:?}", details.package_name, details.target_file);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=patches"); // Rerun if the patches directory changes

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let patches_dir = manifest_dir.join("patches");

    eprintln!("Debug: manifest_dir = {:?}", manifest_dir);
    eprintln!("Debug: patches_dir = {:?}", patches_dir);

    if !patches_dir.exists() {
        eprintln!("Error: No 'patches' directory found at {:?}", patches_dir);
        return Ok(())
    }

    let config_paths = get_config_paths(&patches_dir)
        .with_context(|| format!("Failed to get config paths from {:?}", patches_dir))?;

    if config_paths.is_empty() {
        eprintln!("Error: No edit job files found in {:?}", patches_dir);
        return Ok(())
    }

    for config_path in config_paths {
        eprintln!("\nDebug: Starting code editing process based on {:?}...", config_path);

        let config_content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {:?}", config_path))?;
        let edit_job_config: EditJobConfig = toml::from_str(&config_content)
            .with_context(|| format!("Failed to parse {:?}", config_path))?;

        // Separate search jobs as they don't target a specific file for AST modification
        let mut search_jobs: Vec<&RunSearchDetails> = Vec::new();
        let mut edits_by_file: HashMap<PathBuf, Vec<&EditJob>> = HashMap::new();


        for edit in &edit_job_config.edits {
            match edit {
                EditJob::RunSearch(details) => search_jobs.push(details),
                _ => {
                    let target_file = match edit {
                        EditJob::AddUse(details) => &details.target_file,
                        EditJob::RemoveFunction(details) => &details.target_file,
                        EditJob::ReplaceExpression(details) => &details.target_file,
                        EditJob::AddFunction(details) => &details.target_file,
                        EditJob::AddItem(details) => &details.target_file,
                        EditJob::ReplaceFileContent(details) => &details.target_file,
                        EditJob::ReplaceFileContentFromFile(details) => &details.target_file,
                        EditJob::RemoveCargoDependency(details) => &details.target_file,
                        EditJob::RemoveUse(details) => &details.target_file, // Handle new variant
                        EditJob::RunSearch(_) => unreachable!(), // Handled above
                    };
                    eprintln!("Debug: Adding edit for target_file: {:?}", target_file);
                    edits_by_file.entry(target_file.clone()).or_default().push(edit);
                }
            }
        }

        // Execute search jobs first
        for search_job_details in search_jobs {
            apply_run_search(search_job_details)?;
        }

        for (file_path, edits) in &mut edits_by_file { // Changed to iterate over mutable reference
            eprintln!("\nDebug: Processing file: {:?}", file_path);

            // Special handling for Cargo.toml files for dependency removal
            if file_path.file_name().map_or(false, |name| name == "Cargo.toml") {
                for edit in edits.drain(..) { // Use drain to consume edits from the Vec
                    if let EditJob::RemoveCargoDependency(details) = edit {
                        apply_remove_cargo_dependency(details)?;
                    } else {
                        eprintln!("  Warning: Ignoring non-RemoveCargoDependency edit for Cargo.toml: {:?}", edit);
                    }
                }
                // Check if any Rust code edits are also present for this Cargo.toml file.
                // If yes, process them. Otherwise, continue.
                let has_rust_edits = edits.iter().any(|edit| match edit {
                    EditJob::AddUse(_) 
                    | EditJob::RemoveFunction(_) 
                    | EditJob::ReplaceExpression(_) 
                    | EditJob::AddFunction(_) 
                    | EditJob::AddItem(_) 
                    | EditJob::RemoveUse(_) => true, // Include RemoveUse for Rust files
                    _ => false,
                });

                if !has_rust_edits {
                    continue; // Move to the next file if only Cargo.toml specific edits were processed
                }
            }


            let mut replace_entire_file = false;
            let mut new_file_content_from_string: Option<String> = None;
            let mut new_file_content_from_file: Option<PathBuf> = None;

            for edit in edits.iter() { // Iterate over reference here
                match edit {
                    EditJob::ReplaceFileContent(details) => {
                        replace_entire_file = true;
                        new_file_content_from_string = Some(details.new_content.clone());
                        break; // No need to process other edits if we're replacing the entire file
                    }
                    EditJob::ReplaceFileContentFromFile(details) => {
                        replace_entire_file = true;
                        new_file_content_from_file = Some(details.source_file.clone());
                        break; // No need to process other edits if we're replacing the entire file
                    }
                    _ => {} // Other edits don't trigger full file replacement
                }
            }

            if replace_entire_file {
                if let Some(content) = new_file_content_from_string {
                    eprintln!("Debug: Replacing entire file {:?} with string content.", file_path);
                    apply_replace_file_content(&file_path, &content)?;
                    eprintln!("Successfully replaced content of {:?}", file_path);
                } else if let Some(source_path) = new_file_content_from_file {
                    eprintln!("Debug: Replacing entire file {:?} from source file {:?}.", file_path, source_path);
                    apply_replace_file_content_from_file(&file_path, &source_path)?;
                    eprintln!("Successfully replaced content of {:?} from source file {:?}", file_path, source_path);
                }
            } else {
                let file_content = fs::read_to_string(&file_path)
                    .with_context(|| format!("Failed to read file: {:?}", file_path))?;
                eprintln!("Debug: Original file content for {:?}:\n{}", file_path, file_content);
                let mut ast: syn::File = syn::parse_file(&file_content)
                    .with_context(|| format!("Failed to parse Rust file: {:?}", file_path))?;

                for edit in edits.drain(..) { // Use drain again for Rust edits
                    match edit {
                        EditJob::AddUse(details) => apply_add_use(&mut ast, details)?,
                        EditJob::RemoveFunction(details) => apply_remove_function(&mut ast, details)?,
                        EditJob::ReplaceExpression(details) => apply_replace_expression(&mut ast, details)?,
                        EditJob::AddFunction(details) => apply_add_function(&mut ast, details)?,
                        EditJob::AddItem(details) => apply_add_item(&mut ast, details)?,
                        EditJob::RemoveUse(details) => apply_remove_use(&mut ast, details)?, // New dispatch
                        EditJob::ReplaceFileContent(_) => {{ /* Already handled */ }}
                        EditJob::ReplaceFileContentFromFile(_) => {{ /* Already handled */ }}
                        EditJob::RunSearch(_) => unreachable!(), // Handled above
                        EditJob::RemoveCargoDependency(_) => unreachable!(), // Handled above
                    }
                }

                let formatted_code = prettyplease::unparse(&ast);
                eprintln!("Debug: Formatted code after AST modification for {:?}:\n{}", file_path, formatted_code);

                fs::write(&file_path, &formatted_code)
                    .with_context(|| format!("Failed to write modified file: {:?}", file_path))?;
            }
        }
    }

    eprintln!("\nAll specified edits applied successfully!");
    Ok(())
}