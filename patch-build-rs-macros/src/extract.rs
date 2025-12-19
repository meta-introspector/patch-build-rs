use patch_build_rs_macros::mkbuildrs;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

mkbuildrs! {
    module_name: "extract";
    dependencies: ["proc_macro::TokenStream", "quote::quote", "syn"];
    description: "Extract fixme issues to isolated Nix flake environments";
    exports: ["extract"];
}

pub fn extract_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let fixme_id = input_str.value();
    
    let flake_name = format!("fixme-{}", 
        fixme_id.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .take(20)
            .collect::<String>()
            .to_lowercase()
    );
    
    let flake_content = format!(r#"{{
  description = "Isolated fixme: {}";
  inputs = {{
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  }};
  outputs = {{ self, nixpkgs, rust-overlay }}: {{
    devShells.x86_64-linux.default = pkgs.mkShell {{
      buildInputs = [ rust-bin.stable.latest.default ];
    }};
  }};
}}"#, fixme_id);

    quote! {
        {
            use std::fs;
            let extract_dir = format!("extracted/{}", #flake_name);
            fs::create_dir_all(&extract_dir).unwrap();
            fs::write(format!("{}/flake.nix", extract_dir), #flake_content).unwrap();
            println!("cargo:warning=🔧 Extracted: {}", extract_dir);
        }
    }.into()
}
