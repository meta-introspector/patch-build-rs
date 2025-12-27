fn main() {
    println!("cargo:rerun-if-changed=refactor.toml");

    let config_path = "../refactor.toml";
    if let Err(e) = autowrap_tool::process_refactor_config(config_path) {
        eprintln!("Error processing refactor config: {}", e);
        panic!("Failed to process refactor config");
    }
}