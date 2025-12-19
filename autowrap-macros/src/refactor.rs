use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct RefactorConfig {
    refactor: RefactorMeta,
    splits: Vec<SplitRule>,
    export: ExportConfig,
}

#[derive(Deserialize)]
struct RefactorMeta {
    name: String,
    output_format: String,
}

#[derive(Deserialize)]
struct SplitRule {
    pattern: String,
    wrap_with: Vec<String>,
    imports: Vec<String>,
}

#[derive(Deserialize)]
struct ExportConfig {
    target_dir: String,
    create_tar: bool,
    tar_name: String,
}

pub fn process_refactor_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: RefactorConfig = toml::from_str(&config_str)?;
    
    fs::create_dir_all(&config.export.target_dir)?;
    
    for (i, rule) in config.splits.iter().enumerate() {
        let output = Command::new("rg")
            .args(&["-A", "5", &rule.pattern, ".", "--type", "rust"])
            .output()?;
        
        let matches = String::from_utf8_lossy(&output.stdout);
        let wrapped_code = wrap_code_with_rule(&matches, rule);
        
        let file_path = format!("{}/split_{}.rs", config.export.target_dir, i);
        fs::write(file_path, wrapped_code)?;
    }
    
    if config.export.create_tar {
        Command::new("tar")
            .args(&["-czf", &config.export.tar_name, &config.export.target_dir])
            .output()?;
    }
    
    Ok(())
}

fn wrap_code_with_rule(code: &str, rule: &SplitRule) -> String {
    let imports = rule.imports.join("\n");
    let code_lines: Vec<&str> = code.lines()
        .filter(|line| !line.starts_with("--") && !line.contains(".rs:"))
        .collect();
    
    format!(r#"
prelude! {{
    {}
}}

mkdecl! {{
    {}
}}
"#, imports, code_lines.join("\n"))
}
