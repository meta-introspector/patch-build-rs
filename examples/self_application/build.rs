use patch_build_rs_macros::mkbuildrs;
use mkslop_macros::mkslop;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    mkbuildrs! {
        // Set a cfg flag to indicate that we are in a self-application build
        cfg: "self_application_build" = "true";

        // Add some information about the build environment
        resource_req: {
            ram = "1GB",
            cpu = "1"
        };

        // Specify a secret that might be needed
        secret_req: ["SELF_APPLICATION_TOKEN"];
    }
}
