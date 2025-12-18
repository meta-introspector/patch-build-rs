fn main() {
    println!("Hello from the self-application example!");

    #[cfg(self_application_build)]
    println!("This code is enabled because the 'self_application_build' cfg flag was set by our own build.rs script!");

    #[cfg(not(self_application_build))]
    println!("This code should not be enabled.");

    // You can retrieve the resource requirements and other info passed via `cargo:info`
    // by parsing the build script output, but for this example, we'll just rely on the cfg flag.
}
