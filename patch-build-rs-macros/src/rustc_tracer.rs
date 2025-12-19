use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

pub fn trace_rustc_impl(input: TokenStream) -> TokenStream {
    let _input_str = parse_macro_input!(input as LitStr);
    
    quote! {
        {
            use std::process::Command;
            
            println!("cargo:warning=🔍 Tracing complete rustc system");
            
            // 1. Find rustc binary
            let which_rustc = Command::new("which").arg("rustc").output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "rustc not found".to_string());
            
            // 2. Get real path (follow symlinks)
            let real_path = Command::new("readlink").args(&["-f", &which_rustc]).output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| which_rustc.clone());
            
            // 3. Get rustc version and commit
            let version_output = Command::new("rustc").args(&["--version", "--verbose"]).output()
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .unwrap_or_else(|_| "version unknown".to_string());
            
            // 4. Extract commit hash from version
            let commit_hash = version_output.lines()
                .find(|line| line.starts_with("commit-hash:"))
                .and_then(|line| line.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            // 5. Find source URL from Nix store or GitHub
            let source_url = if !commit_hash.is_empty() && commit_hash != "unknown" {
                format!("https://github.com/rust-lang/rust/archive/{}.tar.gz", commit_hash)
            } else {
                "https://github.com/rust-lang/rust/archive/master.tar.gz".to_string()
            };
            
            // 6. Check for existing source in various locations
            let possible_sources = vec![
                format!("{}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust", 
                        std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())),
                "/nix/store/*rust*/src".to_string(),
                "/usr/src/rust".to_string(),
                "./rust-src".to_string(),
            ];
            
            let mut found_source = None;
            for source_path in &possible_sources {
                if std::path::Path::new(source_path).exists() {
                    found_source = Some(source_path.clone());
                    break;
                }
            }
            
            // 7. Get Nix store info if applicable
            let nix_info = if real_path.starts_with("/nix/store/") {
                let store_path = real_path.split('/').take(4).collect::<Vec<_>>().join("/");
                Command::new("nix").args(&["show-derivation", &store_path]).output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .unwrap_or_else(|_| "nix info unavailable".to_string())
            } else {
                "not in nix store".to_string()
            };
            
            let trace_report = format!(
                r#"
🔍 COMPLETE RUSTC TRACE REPORT

1️⃣ RUSTC BINARY:
   Path: {}
   Real Path: {}
   In Nix Store: {}

2️⃣ VERSION INFO:
{}

3️⃣ SOURCE URL:
   GitHub: {}
   Commit: {}

4️⃣ SOURCE LOCATIONS CHECKED:
{}
   Found: {}

5️⃣ NIX STORE INFO:
{}

6️⃣ NEXT STEPS:
   - Download source: curl -L {} | tar xz
   - Or use rustup: rustup component add rust-src
   - Or use nix: nix-shell -p rustc.src
                "#,
                which_rustc,
                real_path,
                real_path.starts_with("/nix/store/"),
                version_output,
                source_url,
                commit_hash,
                possible_sources.iter().enumerate()
                    .map(|(i, path)| format!("   {}. {}", i+1, path))
                    .collect::<Vec<_>>()
                    .join("\n"),
                found_source.unwrap_or_else(|| "None found".to_string()),
                if nix_info.len() > 100 { 
                    format!("{}...", &nix_info[..100]) 
                } else { 
                    nix_info 
                }
            );
            
            trace_report
        }
    }.into()
}

pub fn custom_rust_driver_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let config = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🚀 Creating custom rust driver");
            
            let driver_code = format!(
                r#"
#!/bin/bash
# Custom Rust Driver - Automated rustc source analysis
# Config: {}

set -e

echo "🚀 Custom Rust Driver Starting..."

# 1. Detect rustc
RUSTC_PATH=$(which rustc)
REAL_RUSTC=$(readlink -f "$RUSTC_PATH")
echo "📍 Rustc found: $REAL_RUSTC"

# 2. Get version and commit
RUSTC_VERSION=$(rustc --version --verbose)
COMMIT_HASH=$(echo "$RUSTC_VERSION" | grep "commit-hash:" | cut -d: -f2 | tr -d ' ')
echo "🔖 Commit: $COMMIT_HASH"

# 3. Determine source URL
if [ -n "$COMMIT_HASH" ] && [ "$COMMIT_HASH" != "unknown" ]; then
    SOURCE_URL="https://github.com/rust-lang/rust/archive/$COMMIT_HASH.tar.gz"
else
    SOURCE_URL="https://github.com/rust-lang/rust/archive/master.tar.gz"
fi
echo "🌐 Source URL: $SOURCE_URL"

# 4. Create workspace
WORKSPACE="./rust-analysis-workspace"
mkdir -p "$WORKSPACE"
cd "$WORKSPACE"

# 5. Download and extract source
if [ ! -d "rust-src" ]; then
    echo "📥 Downloading rust source..."
    curl -L "$SOURCE_URL" | tar xz
    mv rust-* rust-src
    echo "✅ Source extracted to: $(pwd)/rust-src"
fi

# 6. Analyze source structure
echo "🔍 Analyzing source structure..."
cd rust-src

# Count files by type
echo "📊 File analysis:"
find . -name "*.rs" | wc -l | xargs echo "  Rust files:"
find . -name "*.toml" | wc -l | xargs echo "  TOML files:"
find . -name "*.md" | wc -l | xargs echo "  Markdown files:"

# Analyze compiler structure
echo "🧮 Compiler structure:"
ls -la compiler/ 2>/dev/null || echo "  No compiler/ directory"
ls -la src/librustc* 2>/dev/null || echo "  No librustc* directories"
ls -la src/rustc* 2>/dev/null || echo "  No rustc* directories"

# Count keywords in source
echo "🔍 Keyword analysis:"
echo "  fn declarations: $(find . -name "*.rs" -exec grep -c "^fn " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  struct definitions: $(find . -name "*.rs" -exec grep -c "^struct " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  impl blocks: $(find . -name "*.rs" -exec grep -c "^impl " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  trait definitions: $(find . -name "*.rs" -exec grep -c "^trait " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"

# 7. Generate analysis report
cat > ../analysis_report.json << EOF
{{
  "rustc_binary": "$REAL_RUSTC",
  "source_url": "$SOURCE_URL", 
  "commit_hash": "$COMMIT_HASH",
  "source_path": "$(pwd)",
  "analysis_timestamp": "$(date -Iseconds)",
  "workspace": "$WORKSPACE"
}}
EOF

echo "✅ Analysis complete!"
echo "📄 Report saved to: $WORKSPACE/analysis_report.json"
echo "📁 Source available at: $(pwd)"

# 8. Build custom rustc (optional)
if [ "$1" = "--build" ]; then
    echo "🔨 Building custom rustc..."
    ./configure --enable-debug
    make -j$(nproc)
    echo "✅ Custom rustc built!"
fi
                "#, #config
            );
            
            driver_code
        }
    }.into()
}

pub fn auto_source_setup_impl(input: TokenStream) -> TokenStream {
    let _input_str = parse_macro_input!(input as LitStr);
    
    quote! {
        {
            println!("cargo:warning=⚙️ Setting up automatic source analysis");
            
            let setup_code = r#"
// Automatic Rust Source Setup
use std::process::Command;
use std::path::Path;
use std::fs;

pub struct RustSourceManager {
    pub workspace: String,
    pub source_path: Option<String>,
    pub rustc_binary: String,
    pub commit_hash: String,
}

impl RustSourceManager {
    pub fn new() -> Self {
        Self {
            workspace: "./rust-analysis-workspace".to_string(),
            source_path: None,
            rustc_binary: String::new(),
            commit_hash: String::new(),
        }
    }
    
    pub fn detect_rustc(&mut self) -> Result<(), String> {
        // Find rustc binary
        let output = Command::new("which")
            .arg("rustc")
            .output()
            .map_err(|e| format!("Failed to find rustc: {}", e))?;
            
        self.rustc_binary = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        // Get version info
        let version_output = Command::new("rustc")
            .args(&["--version", "--verbose"])
            .output()
            .map_err(|e| format!("Failed to get rustc version: {}", e))?;
            
        let version_str = String::from_utf8_lossy(&version_output.stdout);
        self.commit_hash = version_str.lines()
            .find(|line| line.starts_with("commit-hash:"))
            .and_then(|line| line.split(':').nth(1))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());
            
        Ok(())
    }
    
    pub fn setup_source(&mut self) -> Result<(), String> {
        // Create workspace
        fs::create_dir_all(&self.workspace)
            .map_err(|e| format!("Failed to create workspace: {}", e))?;
            
        let source_dir = format!("{}/rust-src", self.workspace);
        
        if !Path::new(&source_dir).exists() {
            // Download source
            let source_url = if !self.commit_hash.is_empty() && self.commit_hash != "unknown" {
                format!("https://github.com/rust-lang/rust/archive/{}.tar.gz", self.commit_hash)
            } else {
                "https://github.com/rust-lang/rust/archive/master.tar.gz".to_string()
            };
            
            println!("Downloading rust source from: {}", source_url);
            
            let download = Command::new("curl")
                .args(&["-L", &source_url])
                .current_dir(&self.workspace)
                .output()
                .map_err(|e| format!("Failed to download source: {}", e))?;
                
            // Extract
            let extract = Command::new("tar")
                .args(&["xz"])
                .stdin(std::process::Stdio::piped())
                .current_dir(&self.workspace)
                .spawn()
                .and_then(|mut child| {
                    use std::io::Write;
                    child.stdin.as_mut().unwrap().write_all(&download.stdout)?;
                    child.wait()
                })
                .map_err(|e| format!("Failed to extract source: {}", e))?;
                
            // Rename to rust-src
            let entries = fs::read_dir(&self.workspace)
                .map_err(|e| format!("Failed to read workspace: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("rust-") && name != "rust-src" {
                    fs::rename(
                        format!("{}/{}", self.workspace, name),
                        &source_dir
                    ).map_err(|e| format!("Failed to rename source dir: {}", e))?;
                    break;
                }
            }
        }
        
        self.source_path = Some(source_dir);
        Ok(())
    }
    
    pub fn analyze_source(&self) -> Result<SourceAnalysis, String> {
        let source_path = self.source_path.as_ref()
            .ok_or("Source not set up")?;
            
        let mut analysis = SourceAnalysis::new();
        
        // Count files
        analysis.count_files(source_path)?;
        
        // Analyze keywords
        analysis.analyze_keywords(source_path)?;
        
        Ok(analysis)
    }
}

pub struct SourceAnalysis {
    pub rust_files: usize,
    pub total_lines: usize,
    pub keyword_counts: std::collections::HashMap<String, usize>,
}

impl SourceAnalysis {
    pub fn new() -> Self {
        Self {
            rust_files: 0,
            total_lines: 0,
            keyword_counts: std::collections::HashMap::new(),
        }
    }
    
    pub fn count_files(&mut self, source_path: &str) -> Result<(), String> {
        // Implementation for counting files
        Ok(())
    }
    
    pub fn analyze_keywords(&mut self, source_path: &str) -> Result<(), String> {
        // Implementation for keyword analysis
        Ok(())
    }
}
            "#;
            
            setup_code.to_string()
        }
    }.into()
}
