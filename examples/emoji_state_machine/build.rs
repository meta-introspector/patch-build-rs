use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("generated_state.rs");

    let generated_code = "
        use emoji_macros::*;

        #[rocket]
        #[scroll]
        #[reset]
        #[idea]
        #[growth]
        #[money]
        #[robot]
        #[gem]
        #[bullseye]
        #[package]
        #[lock]
        #[web]
        #[moai]
        #[temple]
        #[ballot]
        #[halo]
        #[sparkle]
        #[magnifying_glass]
        #[forty_two]
        #[truck]
        #[octopus]
        #[fork_knife]
        #[herb]
        #[arrows]
        #[bandage]
        #[dna]
        #[up_arrow]
        #[broom]
        #[map]
        #[chart]
        #[owl]
        #[notes]
        pub struct GeneratedState {
            // This struct is now generated with all the fields and methods
            // from the applied emoji macros.
        }
    ";

    fs::write(&dest_path, generated_code).unwrap();
}

