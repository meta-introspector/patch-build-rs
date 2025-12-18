use patch_build_rs_macros::{lru, mkbuildrs};
use mkslop_macros::mkslop;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[lru(capacity = 2, cache_dir = "my_build_cache")]
fn get_project_version(project_name: String, path: PathBuf) -> String {
    println!("--- Running get_project_version for {} ---", project_name);
    // Simulate some work, e.g., reading a file
    let content = fs::read_to_string(path.join("Cargo.toml"))
        .unwrap_or_else(|_| format!("version_not_found_for_{}", project_name));
    
    // Simple parsing to get version
    for line in content.lines() {
        if line.trim().starts_with("version = \"") {
            return line.split('\"').nth(1).unwrap_or("0.0.0").to_string();
        }
    }
    "0.0.0".to_string()
}

fn main() {
    // A simple build script that prints a message
    println!("cargo:rerun-if-changed=build.rs");
    println!("Hello from build.rs!");

    // Use mkbuildrs! to set some config flags and resource requirements
    mkbuildrs! {
        cfg: "example_build" = "true";
        resource_req: {
            ram = "512MB",
            cpu = "1"
        };
    }

    let current_dir = env::current_dir().unwrap();

    let version = get_project_version(
        "basic-build-script-example".to_string(),
        current_dir.clone(),
    );
    println!("cargo:info=Project Version: {}", version);

    // Call it again to demonstrate caching
    let version_cached = get_project_version(
        "basic-build-script-example".to_string(),
        current_dir.clone(),
    );
    println!("cargo:info=Project Version (cached): {}", version_cached);

    // Simulate another project
    let another_version = get_project_version(
        "mkslop-core".to_string(),
        current_dir.join("../mkslop-core"),
    );
    println!("cargo:info=Another Project Version: {}", another_version);

    // Call it again for another project to demonstrate capacity
    let another_version_cached = get_project_version(
        "mkslop-core".to_string(),
        current_dir.join("../mkslop-core"),
    );
    println!("cargo:info=Another Project Version (cached): {}", another_version_cached);
}