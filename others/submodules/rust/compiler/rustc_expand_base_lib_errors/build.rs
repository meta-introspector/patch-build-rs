use std::{env, fs, path::{Path, PathBuf}};
use syn::{parse_file, Item};
use quote::quote;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let src_dir = manifest_dir.join("src");

    fs::create_dir_all(&out_dir)?;

    let mut generated_includes = String::new();
    let mut generated_pub_uses = String::new();

    // Only process our specific struct files for now
    let files_to_process = vec!["trace_macro_base.rs", "trace_macro_note.rs"];

    for file_name in files_to_process {
        let path = src_dir.join(file_name);
        println!("cargo:rerun-if-changed={}", path.display());
        let original_code = fs::read_to_string(&path)?;
        let file_stem = path.file_stem().ok_or("No file stem")?.to_string_lossy();

        let ast = parse_file(&original_code)?;
        let mut transformed_items = Vec::new();

        let mut struct_names_in_file = Vec::new(); // Collect struct names for pub use

        for item in ast.items {
            let wrapped_item = match item {
                Item::Struct(item_struct) => {
                    struct_names_in_file.push(item_struct.ident.clone());
                    quote! { wrap_struct! { #item_struct } }
                },
                _ => quote! { #item }, // Pass through other items without wrapping
            };
            transformed_items.push(wrapped_item);
        }

        let transformed_file_content = quote! {
            #(#transformed_items)*
        }.to_string();

        let output_file_name = format!("transformed_{}.rs", file_stem);
        let output_path = out_dir.join(&output_file_name);
        fs::write(&output_path, transformed_file_content)?;

        // Add include! for this transformed file
        generated_includes.push_str(&format!("pub mod {};\n", output_file_name.replace(".rs", "")));

        // Add pub use for structs found in this file
        for struct_name in struct_names_in_file {
            generated_pub_uses.push_str(&format!("pub use {}::{};\n", output_file_name.replace(".rs", ""), struct_name));
        }
    }

    // Now, create the actual lib.rs for this crate that includes the transformed files
    // and re-exports items from the original lib.rs (which has the traits)
    let original_lib_rs_path = src_dir.join("lib.rs");
    println!("cargo:rerun-if-changed={}", original_lib_rs_path.display());
    let original_lib_rs_content = fs::read_to_string(&original_lib_rs_path)?;

    let wrapping_macros_str = r#"\
        macro_rules! wrap_struct {
            ($($item:item)*) => { $(
                #[cfg(debug_assertions)]
                eprintln!("Wrapped struct: {}", stringify!($item));
                $item
            )* };
        }
        // Add other wrapping macros (fn, enum, etc.) if needed for this crate
    "#;

    let final_lib_rs_content = format!(
        "{}\n{}\n{}",
        wrapping_macros_str,
        generated_includes,
        generated_pub_uses
    );

    fs::write(out_dir.join("lib.rs"), final_lib_rs_content)?;

    Ok(())
}
