// grast: Greppable AST CLI tool

use grast_core::GrastDb;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use syn::parse_file; // Explicitly import parse_file
use walkdir::WalkDir;

// Function to process a single Rust file and print its Turtle representation
fn process_file_and_print_turtle(file_path: &Path) -> Result<()> {
    let code = fs::read_to_string(file_path)
        .context(format!("Failed to read Rust file: {}", file_path.display()))?;
    let ast = parse_file(&code)
        .context(format!("Failed to parse Rust code from: {}", file_path.display()))?;
    let mut db = GrastDb::new();
    db.flatten(&ast);
    print!("{}", db.to_turtle());
    Ok(())
}

// CLI interface
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: grast <file.rs>        # flatten to turtle");
        eprintln!("       grast <directory>      # flatten all .rs files in directory to turtle");
        eprintln!("       grast -u <file.turtle> # unflatten from turtle");
        eprintln!("       grast --vfs <file.rs> <dir>  # export to VFS");
        anyhow::bail!("Incorrect usage.");
    }
    
    match args[1].as_str() {
        "-u" | "--ungrast" => {
            if args.len() < 3 {
                anyhow::bail!("Usage: grast -u <file.turtle>");
            }
            let input = fs::read_to_string(&args[2])
                .context(format!("Failed to read turtle file: {}", &args[2]))?;
            let db = GrastDb::from_turtle(&input);
            println!("Loaded {} triples", db.triples.len());
            // In a real ungrast, you'd convert back to Rust code and print/write it
            // For now, we just confirm loading.
            println!("{}", db.to_turtle()); // Print the re-serialized triples
        }
        "--vfs" => {
            if args.len() < 4 {
                anyhow::bail!("Usage: grast --vfs <file.rs> <dir>");
            }
            let code = fs::read_to_string(&args[2])
                .context(format!("Failed to read Rust file: {}", &args[2]))?;
            let ast = parse_file(&code)
                .context("Failed to parse Rust code")?;
            let mut db = GrastDb::new();
            db.flatten(&ast);
            db.to_vfs(Path::new(&args[3]))
                .context(format!("Failed to export to VFS at {}", &args[3]))?;
            println!("Exported to VFS at {}", args[3]);
        }
        input_path_str => {
            let input_path = PathBuf::from(input_path_str);
            if input_path.is_dir() {
                for entry in WalkDir::new(&input_path).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                        process_file_and_print_turtle(path)
                            .unwrap_or_else(|e| eprintln!("Error processing {}: {}", path.display(), e));
                    }
                }
            } else if input_path.is_file() {
                process_file_and_print_turtle(&input_path)?;
            } else {
                anyhow::bail!("Input path '{}' is neither a file nor a directory.", input_path.display());
            }
        }
    }

    Ok(())
}