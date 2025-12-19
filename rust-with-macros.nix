{ pkgs ? import <nixpkgs> {} }:

let
  rustSource = pkgs.fetchFromGitHub {
    owner = "rust-lang";
    repo = "rust";
    rev = "master";
    sha256 = "0000000000000000000000000000000000000000000000000000";
  };
  
  patchedRustSource = pkgs.runCommand "patched-rust-source" {} ''
    cp -r ${rustSource} $out
    chmod -R +w $out
    
    # Inject our macros into all build.rs files
    find $out -name "build.rs" -type f | while read buildrs; do
      echo 'use patch_build_rs_macros::{mkbuildrs, autowrap};' > "$buildrs.new"
      echo 'mkbuildrs! { original_build: r#"' >> "$buildrs.new"
      cat "$buildrs" >> "$buildrs.new"
      echo '"#; }' >> "$buildrs.new"
      mv "$buildrs.new" "$buildrs"
    done
    
    # Replace bootstrap.py
    cat > $out/src/bootstrap/bootstrap.py << 'EOF'
#!/usr/bin/env python3
# Macro-wrapped bootstrap
import sys
STAGES = {"stage0": ["std"], "stage1": ["rustc"], "stage2": ["tools"]}
stage = sys.argv[1] if len(sys.argv) > 1 else "stage2"
print(f"Macro-injected build stage: {stage}")
EOF
  '';

in pkgs.rustPlatform.buildRustPackage {
  pname = "rust-with-macros";
  version = "macro-injected";
  
  src = patchedRustSource;
  
  cargoSha256 = "0000000000000000000000000000000000000000000000000000";
  
  buildInputs = with pkgs; [
    llvm
    cmake
    python3
  ];
  
  configurePhase = ''
    export RUST_BACKTRACE=1
    python3 src/bootstrap/bootstrap.py stage2
  '';
}
