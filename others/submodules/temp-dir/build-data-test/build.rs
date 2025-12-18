fn main() {
    build_data::set_SOURCE_DATE().unwrap();
    build_data::set_SOURCE_TIME().unwrap();
    build_data::set_SOURCE_TIMESTAMP().unwrap();
    build_data::set_SOURCE_EPOCH_TIME().unwrap();
    build_data::set_BUILD_DATE();
    build_data::set_BUILD_TIME();
    build_data::set_BUILD_TIMESTAMP();
    build_data::set_BUILD_EPOCH_TIME();
    build_data::set_BUILD_HOSTNAME().unwrap();
    build_data::set_GIT_BRANCH().unwrap();
    build_data::set_GIT_COMMIT().unwrap();
    build_data::set_GIT_COMMIT_SHORT().unwrap();
    build_data::set_GIT_DIRTY().unwrap();
    build_data::set_RUSTC_VERSION().unwrap();
    build_data::set_RUSTC_VERSION_SEMVER().unwrap();
    build_data::set_RUST_CHANNEL().unwrap();
    build_data::set_TARGET_PLATFORM().unwrap();

    // Cargo sets some variables automatically:
    // - CARGO_PKG_VERSION
    // - CARGO_PKG_NAME
    // - CARGO_CRATE_NAME
    // - Many others:
    //   https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

    // Speed up dev builds.
    build_data::no_debug_rebuilds().unwrap();
}
