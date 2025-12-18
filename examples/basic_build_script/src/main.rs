// examples/basic_build_script.rs
// This file demonstrates the usage of the mkbuildrs! macro.

use patch_build_rs_macros::mkbuildrs;
use mkslop_macros::mkslop;

fn main() {
    // Generate build.rs content using mkbuildrs! macro
    mkbuildrs! {
        check_cfg: "feature", values = ["my_feature", "another_feature"];
        cfg: "my_custom_flag" = "enabled";
        resource_req: { ram = "8GB", cpu = "4", instance_type = "c5.xlarge" };
        secret_req: ["DATABASE_URL", "API_TOKEN"];
    }

    // The macro expands to println! statements that are typically run by Cargo
    // when a build.rs script is executed.
    // To see the output, you would normally run this as a build.rs file.
    // For this example, we're simulating its output.
    
    // In a real build.rs, there would be no explicit main function call;
    // the macro expansion directly becomes the body of build.rs
    // For demonstration, we just show the expanded calls.
}
