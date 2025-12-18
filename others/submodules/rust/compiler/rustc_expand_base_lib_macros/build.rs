fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=rustc_expand_base_lib_macros build script running!");
}
