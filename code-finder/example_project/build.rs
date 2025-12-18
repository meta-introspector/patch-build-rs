use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Rerun build script if these files change
    println!("cargo:rerun-if-changed=../../find-code.sh");
    println!("cargo:rerun-if-changed=../../generate-macro-calls.sh");

    // Path to the root of the project
    let project_root = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("../.."); // Adjust this path if example_project is not two levels deep from the root

    // Construct the command to run find-code.sh and pipe its output to generate-macro-calls.sh
    let find_code_script = project_root.join("code-finder/find-code.sh");
    let generate_macros_script = project_root.join("code-finder/generate-macro-calls.sh");

    let find_code_output = Command::new(find_code_script)
        .current_dir(&project_root) // Run from project root
        .output()
        .expect("Failed to execute find-code.sh");

    if !find_code_output.status.success() {
        eprintln!("find-code.sh failed: {}", String::from_utf8_lossy(&find_code_output.stderr));
        panic!("find-code.sh failed");
    }

    let mut generate_macros_cmd = Command::new(generate_macros_script)
        .current_dir(&project_root) // Run from project root
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn generate-macro-calls.sh");

    {
        let stdin = generate_macros_cmd.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(&find_code_output.stdout).expect("Failed to write to stdin of generate-macro-calls.sh");
    } // stdin is closed here, signaling EOF

    let generate_macros_output = generate_macros_cmd.wait_with_output().expect("Failed to wait for generate-macro-calls.sh");

    if !generate_macros_output.status.success() {
        eprintln!("generate-macro-calls.sh failed: {}", String::from_utf8_lossy(&generate_macros_output.stderr));
        panic!("generate-macro-calls.sh failed");
    }

    let macro_calls_rust_code = String::from_utf8_lossy(&generate_macros_output.stdout);

    // Write the generated macro calls to a temporary file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("generated_macro_calls.rs");
    std::fs::write(&dest_path, macro_calls_rust_code.as_bytes()).expect("Failed to write generated macro calls to file");

    println!("cargo:rustc-env=GENERATED_MACRO_CALLS_PATH={}", dest_path.display());
    println!("cargo:rustc-link-search=native={}", out_dir);

    // In a real scenario, you would probably include this generated file
    // in your main crate, e.g., using `include!(concat!(env!("OUT_DIR"), "/generated_macro_calls.rs"));`
    // However, the purpose of `process_match!` is to directly transform at compile time
    // using the proc-macro.
    // For demonstration purposes, the macro_lib::process_match! macro itself
    // might directly influence the build output or generate new files.
    // Since we're piping the output into a file, the actual processing would be
    // done by the `process_match!` macro during the compilation of the `build.rs`
    // if it were directly called here.

    // To actually *run* the macros during build.rs compilation, we need to
    // depend on the macro_lib and invoke them.
    // For now, the `generate-macro-calls.sh` script produces the macro calls,
    // and `build.rs` just writes them to a file.
    // A subsequent step would be to *include* this file and *invoke* the macros.
}
