use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write; // Import Write trait for fs::File

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write expand_replacement.rs
    let dest_path_replacement = out_dir.join("expand_replacement.rs");
    let mut file_replacement = fs::File::create(&dest_path_replacement).unwrap();
    let content_replacement = fs::read_to_string("./expand_replacement_content.rs").unwrap();
    file_replacement.write_all(content_replacement.as_bytes()).unwrap();

    // Create an empty expand_module.rs to satisfy the include! in lib.rs
    let dest_path_module = out_dir.join("expand_module.rs");
    fs::File::create(&dest_path_module).unwrap();
}
