use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("generated_state.rs");

    let generated_code = "
        use rocket_macro::rocket;
        use scroll_macro::scroll;
        use reset_macro::reset;
        use idea_macro::idea;
        use growth_macro::growth;
        use money_macro::money;
        use robot_macro::robot;
        use gem_macro::gem;
        use bullseye_macro::bullseye;
        use package_macro::package;
        use lock_macro::lock;
        use web_macro::web;
        use moai_macro::moai;
        use temple_macro::temple;
        use ballot_macro::ballot;
        use halo_macro::halo;
        use sparkle_macro::sparkle;
        use magnifying_glass_macro::magnifying_glass;
        use forty_two_macro::forty_two;
        use truck_macro::truck;
        use octopus_macro::octopus;
        use fork_knife_macro::fork_knife;
        use herb_macro::herb;
        use arrows_macro::arrows;
        use bandage_macro::bandage;
        use dna_macro::dna;
        use up_arrow_macro::up_arrow;
        use broom_macro::broom;
        use map_macro::map;
        use chart_macro::chart;
        use owl_macro::owl;
        use notes_macro::notes;

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

