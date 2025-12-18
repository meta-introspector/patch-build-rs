use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join("generated.rs");
    let mut generated_code = String::new();

    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");

    let mut file_names = Vec::new();
    let mut test_routines = Vec::new();

    // Iterate over files in the src directory
    for entry in fs::read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name_os) = path.file_stem() {
                if let Some(file_name) = file_name_os.to_str() {
                    // Collect file names for introspective interface
                    file_names.push(format!("\"{}\"", file_name));

                    // Generate 'mod' statements for .rs files (assuming they are modules)
                    if path.extension().map_or(false, |ext| ext == "rs") && file_name != "lib" {
                        // Generate a simple test routine for each module
                        test_routines.push(format!(r###"
                            #[test]
                            fn test_module_{0}() {{
                                // A placeholder test: ensures the module can be referred to.
                                // Further specific tests would be added manually.
                                let _ = crate::{1};
                                assert!(true);
                            }}
			    "###,
                            file_name,
                            file_name
                        ));
                    }
                }
            }
        }
    }

    // Generate introspective interface
    generated_code.push_str(&format!(r###"
        /// Returns a list of all source file names in this crate's src directory.
        pub fn get_src_file_names() -> Vec<&'static str> {{
            vec![{}]
        }}
        "###,
        file_names.join(", ")
    ));



    // Add test routines
    generated_code.push_str("#[cfg(test)]\nmod generated_tests {
");
    for test_routine in test_routines {
        generated_code.push_str(&test_routine);
        generated_code.push_str("\n");
    }
    generated_code.push_str("}
");


    fs::write(&dest_path, &generated_code).unwrap();
    // eprintln!("GENERATED CODE:\n{}", generated_code); // Commented out for normal operation
    // panic!("Debugging build.rs output"); // Commented out for normal operation
    // Invalidate the build if the src directory changes - Uncommented for normal operation
    println!("cargo:rerun-if-changed={}", src_dir.display());
}
