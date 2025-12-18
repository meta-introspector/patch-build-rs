use std::{env, fs, path::{Path, PathBuf}};
use syn::{parse_file, Item, Visibility, parse_quote};
use quote::quote;
use proc_macro2::TokenStream as ProcMacro2TokenStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let src_dir = manifest_dir.join("src");

    fs::create_dir_all(&out_dir)?;

    let mut generated_lib_rs_content = String::new();
    let mut all_transformed_file_includes = std::collections::BTreeSet::new();
    let mut file_transformation_counter = 0; // Counter for unique filenames derived from original files

    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("cargo:rerun-if-changed={}", path.display());
            let original_code = fs::read_to_string(&path)?;
            let file_stem = path.file_stem().ok_or("No file stem")?.to_string_lossy().to_string();

            let mut ast: syn::File = parse_file(&original_code)?;
            let mut modified_items: Vec<syn::Item> = Vec::new();

            // Apply MODULE_HEADER! at the beginning of the generated file
            let header_tokens: ProcMacro2TokenStream = quote! { macro_wrapper_lib::MODULE_HEADER!(); };
            modified_items.push(parse_quote! { #header_tokens });


            for item in ast.items {
                let wrapped_item: syn::Item = match item {
                    syn::Item::Fn(mut item_fn) => {
                        if matches!(item_fn.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_fn] #item_fn }
                        } else {
                            parse_quote! { #item_fn }
                        }
                    },
                    syn::Item::Struct(mut item_struct) => {
                        if matches!(item_struct.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_struct] #item_struct }
                        } else {
                            parse_quote! { #item_struct }
                        }
                    },
                    syn::Item::Enum(mut item_enum) => {
                        if matches!(item_enum.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_enum] #item_enum }
                        } else {
                            parse_quote! { #item_enum }
                        }
                    },
                    syn::Item::Trait(mut item_trait) => {
                        if matches!(item_trait.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_trait] #item_trait }
                        } else {
                            parse_quote! { #item_trait }
                        }
                    },
                    syn::Item::Impl(item_impl) => {
                        // Impl blocks are always wrapped to allow inspection
                        parse_quote! { #[macro_wrapper_lib::wrap_impl] #item_impl }
                    },
                    syn::Item::Mod(mut item_mod) => {
                        if matches!(item_mod.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_mod] #item_mod }
                        } else {
                            parse_quote! { #item_mod }
                        }
                    },
                    syn::Item::Static(mut item_static) => {
                        if matches!(item_static.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_static] #item_static }
                        } else {
                            parse_quote! { #item_static }
                        }
                    },
                    syn::Item::Const(mut item_const) => {
                        if matches!(item_const.vis, Visibility::Public(_)) {
                            parse_quote! { #[macro_wrapper_lib::wrap_const] #item_const }
                        } else {
                            parse_quote! { #item_const }
                        }
                    },
                    syn::Item::Use(item_use) => {
                        // Use statements are always wrapped
                        parse_quote! { #[macro_wrapper_lib::wrap_use] #item_use }
                    },
                    _ => {
                        // Catch-all for other items
                        parse_quote! { #[macro_wrapper_lib::wrap_item] #item }
                    },
                };
                modified_items.push(wrapped_item);
            }

            // Apply MODULE_FOOTER! at the end of the generated file
            let footer_tokens: ProcMacro2TokenStream = quote! { macro_wrapper_lib::MODULE_FOOTER!(); };
            modified_items.push(parse_quote! { #footer_tokens });


            // This will generate one output file per original source file.
            // Each output file will contain all its items wrapped with the attributes.
            let output_file_name = format!("transformed_{}_{}.rs", file_stem, file_transformation_counter);
            file_transformation_counter += 1; // Increment for the next file
            let output_path = out_dir.join(&output_file_name);

            let final_file_content = quote! { #(#modified_items)* }.to_string();
            fs::write(&output_path, final_file_content)?;
            all_transformed_file_includes.insert(format!("include!(\"{}\");", output_path.display()));
        }
    }

    // generated_lib_rs_content will now contain include statements for the transformed files.
    for include_statement in all_transformed_file_includes {
        generated_lib_rs_content.push_str(&format!("{}\n", include_statement));
    }
    fs::write(out_dir.join("lib.rs"), generated_lib_rs_content)?;

    Ok(())
}
