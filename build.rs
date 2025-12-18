use patch_build_rs_macros::mkbuildrs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Using mkbuildrs! to define metadata about the patch-build-rs workspace itself.
    mkbuildrs! {
        // A cfg flag to indicate this is the root build of the patching system.
        cfg: "patch_build_rs_workspace" = "true";

        // Declare the components of the patching system as features.
        check_cfg: "feature", values = ["mkslop", "nix2proc", "patch-build-rs-macros"];

        // Set features based on the presence of the macro crates.
        cfg: "feature" = "mkslop";
        cfg: "feature" = "nix2proc";
        cfg: "feature" = "patch-build-rs-macros";

        // Define resource requirements for building the entire workspace.
        resource_req: {
            ram = "4GB",
            cpu = "2",
            instance_type = "t3.medium"
        };

        // A placeholder for a secret that might be used in the build process.
        secret_req: ["PATCH_BUILD_RS_CI_TOKEN"];
    }
}
