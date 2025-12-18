use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_lib_path = out_dir.join("lib.rs"); // This will be the generated lib.rs

    let mut generated_mods = String::new();
    let src_meta_macros_defs_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/meta_macros_defs");

    eprintln!("--- rustc_expand_meta_macros build.rs running ---");
    eprintln!("Generating pub mod statements and copying files for directory: {}", src_meta_macros_defs_dir.display());

    if let Ok(entries) = fs::read_dir(&src_meta_macros_defs_dir) {
        let mut module_names: Vec<String> = Vec::new(); // Collect module names

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if file_name.ends_with(".rs") && file_name != "mod.rs" {
                            let module_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                            module_names.push(module_name.clone());

                            // Copy the .rs file to OUT_DIR
                            let dest_file_path = out_dir.join(&path.file_name().unwrap());
                            fs::copy(&path, &dest_file_path).unwrap();
                            eprintln!("Copied {} to {}", path.display(), dest_file_path.display());
                        }
                    }
                }
            }
        }
        
        // Sort module names to ensure consistent output order for pub mod statements
        module_names.sort();
        for module_name in module_names {
            generated_mods.push_str(&format!("pub mod {};\n", module_name));
        }
    }

    // Write the generated pub mod statements to OUT_DIR/lib.rs
    fs::write(&dest_lib_path, &generated_mods).unwrap();

    eprintln!("--- Generated lib.rs content ---");
    eprintln!("{}", generated_mods);
    eprintln!("--- End of generated lib.rs content ---");

    // Invalidate the build if the src/meta_macros_defs directory or build.rs itself changes
    println!("cargo:rerun-if-changed=src/meta_macros_defs");
    println!("cargo:rerun-if-changed=build.rs");
}