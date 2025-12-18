use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use rust_nix_bootstrap::{NixPathsProvider, HardcodedNixPaths};

#[derive(Debug, Clone)]
struct NixPaths {
    glibc_dev: String,
    gcc_path: String,
    gcc_cpp_include: String,
        openssl_include: String, // Path to OpenSSL development headers
}

// Helper function to get Nix path from environment variable or hardcoded fallback
fn get_nix_path_from_env(env_var: &str, hardcoded_path_from_provider: &'static str) -> String {
    let env_val = env::var(env_var);

    match env_val {
        Ok(path_from_env) => {
            // Compare with hardcoded path if available
            if path_from_env != hardcoded_path_from_provider {
                println!(
                    "cargo:warning=Nix path mismatch for {}: Environment provided '{}', hardcoded is '{}'.",
                    env_var, path_from_env, hardcoded_path_from_provider
                );
                println!("cargo:warning=Using path from environment variable: {}", path_from_env);
            } else {
                println!("cargo:warning=Nix path for {} matches hardcoded: {}", env_var, path_from_env);
            }
            path_from_env
        }
        Err(_) => {
            println!(
                "cargo:warning=Environment variable {} not set. Using hardcoded fallback path from provider: {}",
                env_var, hardcoded_path_from_provider
            );
            hardcoded_path_from_provider.to_string()
        }
    }
}

impl NixPaths {
    fn default_nix_paths() -> Self {
        let hardcoded_provider = HardcodedNixPaths;
        NixPaths {
            glibc_dev: get_nix_path_from_env(
                "NIX_GLIBC_DEV",
                hardcoded_provider.glibc_dev(),
            ),
            gcc_path: get_nix_path_from_env(
                "NIX_GCC_PATH",
                hardcoded_provider.gcc_path(),
            ),
            gcc_cpp_include: get_nix_path_from_env(
                "NIX_GCC_CPP_INCLUDE",
                hardcoded_provider.gcc_cpp_include(),
            ),
            openssl_include: get_nix_path_from_env(
                "NIX_OPENSSL_INCLUDE",
                hardcoded_provider.openssl_include(),
            ),
        }
    }
}

/// Tries to use system libgit2 and emits necessary build script instructions.
fn try_system_libgit2() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg.range_version("1.9.0".."1.10.0").probe("libgit2") {
        Ok(lib) => {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            Ok(lib)
        }
        Err(e) => {
            println!("cargo:warning=failed to probe system libgit2: {e}");
            Err(e)
        }
    }
}

fn main() {
    // Aggressively unset potentially interfering environment variables
    env::remove_var("CXX");
    env::remove_var("CXXFLAGS");
    env::remove_var("CPATH");
    env::remove_var("C_INCLUDE_PATH");
    env::remove_var("CPLUS_INCLUDE_PATH");
    env::remove_var("CC"); // Unset CC as well, will set it explicitly later for cc::Build
    env::remove_var("CFLAGS");
    env::remove_var("LIBRARY_PATH");
    env::remove_var("LD_LIBRARY_PATH");
    env::remove_var("PROTOC");
    env::remove_var("PROTOC_INCLUDE");
    env::remove_var("BINDGEN_EXTRA_CLANG_ARGS");
    env::remove_var("LLVM_CONFIG");
    env::remove_var("LLVM_CONFIG_PATH");
    env::remove_var("LIBCLANG_PATH");
    env::remove_var("LIBCLANG_FLAGS");
    env::remove_var("NIX_GLIBC_DEV");
    env::remove_var("NIX_GCC_PATH");
    env::remove_var("NIX_GCC_REAL_PATH");
    env::remove_var("PKG_CONFIG_PATH");

    let nix_paths = NixPaths::default_nix_paths();

    println!(
        "cargo:rustc-check-cfg=cfg(\
            libgit2_vendored,\
        )"
    );

    let https = env::var("CARGO_FEATURE_HTTPS").is_ok();
    let ssh = env::var("CARGO_FEATURE_SSH").is_ok();
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();
    let zlib_ng_compat = env::var("CARGO_FEATURE_ZLIB_NG_COMPAT").is_ok();

    // Specify `LIBGIT2_NO_VENDOR` to force to use system libgit2.
    // Due to the additive nature of Cargo features, if some crate in the
    // dependency graph activates `vendored` feature, there is no way to revert
    // it back. This env var serves as a workaround for this purpose.
    println!("cargo:rerun-if-env-changed=LIBGIT2_NO_VENDOR");
    let forced_no_vendor = env::var_os("LIBGIT2_NO_VENDOR").map_or(false, |s| s != "0");

    if forced_no_vendor {
        if try_system_libgit2().is_err() {
            panic!(
                "\
The environment variable `LIBGIT2_NO_VENDOR` has been set but no compatible system libgit2 could be found.
The build is now aborting. To disable, unset the variable or use `LIBGIT2_NO_VENDOR=0`.
",
            );
        }

        // We've reached here, implying we're using system libgit2.
        return;
    }

    // To use zlib-ng in zlib-compat mode, we have to build libgit2 ourselves.
    let try_to_use_system_libgit2 = !vendored && !zlib_ng_compat;
    if try_to_use_system_libgit2 && try_system_libgit2().is_ok() {
        // using system libgit2 has worked
        return;
    }

    println!("cargo:rustc-cfg=libgit2_vendored");

    if !Path::new("libgit2/src").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libgit2"])
            .status();
    }

    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let mut cfg = cc::Build::new();
    fs::create_dir_all(&include).unwrap();

    // Explicitly set the C compiler
    cfg.compiler(&format!("{}/bin/gcc", nix_paths.gcc_path));
    // Explicitly set the CXX environment variable for C++ compilation (cc::Build will use it)
    env::set_var("CXX", &format!("{}/bin/g++", nix_paths.gcc_path));

    // Explicitly add include paths
    cfg.flag(&format!("-isystem{}/include", nix_paths.glibc_dev)); // Glibc C headers
    cfg.include(&nix_paths.gcc_cpp_include); // GCC C++ headers
       cfg.include(&nix_paths.openssl_include); // OpenSSL headers
    cfg.flag("-include").flag("stdint.h"); // Force-include stdint.h

    // Copy over all header files
    cp_r("libgit2/include", &include);


    cfg.include(&include)
        .include("libgit2/src/libgit2")
        .include("libgit2/src/util")
        .include("libgit2/src/util/hash") // Add this line
        .include("libgit2/src/util/hash/sha1dc") // Added for git_hash_sha1_ctx definition
        .out_dir(dst.join("build"))
        .warnings(false);

    // Include all cross-platform C files
    add_c_files(&mut cfg, "libgit2/src/libgit2");
    add_c_files(&mut cfg, "libgit2/src/util");

    // These are activated by features, but they're all unconditionally always
    // compiled apparently and have internal #define's to make sure they're
    // compiled correctly.
    add_c_files(&mut cfg, "libgit2/src/libgit2/transports");
    add_c_files(&mut cfg, "libgit2/src/libgit2/streams");

    // Always use bundled HTTP parser (llhttp) for now
    cfg.include("libgit2/deps/llhttp");
    add_c_files(&mut cfg, "libgit2/deps/llhttp");

    // external/system xdiff is not yet supported
    cfg.include("libgit2/deps/xdiff");
    add_c_files(&mut cfg, "libgit2/deps/xdiff");

    // Use the included PCRE regex backend.
    //
    // Ideally these defines would be specific to the pcre files (or placed in
    // a config.h), but since libgit2 already has a config.h used for other
    // reasons, just define on the command-line for everything. Perhaps there
    // is some way with cc to have different instructions per-file?
    cfg.define("GIT_REGEX_BUILTIN", "1")
        .include("libgit2/deps/pcre")
        .define("HAVE_STDINT_H", Some("1"))
        .define("HAVE_MEMMOVE", Some("1"))
        .define("GIT_OPENSSL", Some("1")) // Use GIT_OPENSSL for SHA operations


        .define("NO_RECURSE", Some("1"))
        .define("NEWLINE", Some("10"))
        .define("POSIX_MALLOC_THRESHOLD", Some("10"))
        .define("LINK_SIZE", Some("2"))
        .define("PARENS_NEST_LIMIT", Some("250"))
        .define("MATCH_LIMIT", Some("10000000"))
        .define("MATCH_LIMIT_RECURSION", Some("MATCH_LIMIT"))
        .define("MAX_NAME_SIZE", Some("32"))
        .define("MAX_NAME_COUNT", Some("10000"));
    // "no symbols" warning on pcre_string_utils.c is because it is only used
    // when when COMPILE_PCRE8 is not defined, which is the default.
    add_c_files(&mut cfg, "libgit2/deps/pcre");

    cfg.file("libgit2/src/util/allocators/failalloc.c");
    cfg.file("libgit2/src/util/allocators/stdalloc.c");

    if windows {
        add_c_files(&mut cfg, "libgit2/src/util/win32");
        cfg.define("STRSAFE_NO_DEPRECATE", None);
        cfg.define("WIN32", None);
        cfg.define("_WIN32_WINNT", Some("0x0600"));

        // libgit2's build system claims that forks like mingw-w64 of MinGW
        // still want this define to use C99 stdio functions automatically.
        // Apparently libgit2 breaks at runtime if this isn't here? Who knows!
        if target.contains("gnu") {
            cfg.define("__USE_MINGW_ANSI_STDIO", "1");
        }
    } else {
        add_c_files(&mut cfg, "libgit2/src/util/unix");
        cfg.flag("-fvisibility=hidden");
    }
    if target.contains("solaris") || target.contains("illumos") {
        cfg.define("_POSIX_C_SOURCE", "200112L");
        cfg.define("__EXTENSIONS__", None);
    }

    let mut features = String::new();

    features.push_str("#ifndef INCLUDE_features_h\n");
    features.push_str("#define INCLUDE_features_h\n");
    features.push_str("#define GIT_THREADS 1\n");
    features.push_str("#define GIT_TRACE 1\n");
    features.push_str("#define GIT_HTTPPARSER_BUILTIN 1\n");

    if !target.contains("android") {
        features.push_str("#define GIT_USE_NSEC 1\n");
    }

    if windows {
        features.push_str("#define GIT_IO_WSAPOLL 1\n");
    } else {
        // Should we fallback to `select` as more systems have that?
        features.push_str("#define GIT_IO_POLL 1\n");
        features.push_str("#define GIT_IO_SELECT 1\n");
    }

    if target.contains("apple") {
        features.push_str("#define GIT_USE_STAT_MTIMESPEC 1\n");
    } else {
        features.push_str("#define GIT_USE_STAT_MTIM 1\n");
    }

    if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32" {
        features.push_str("#define GIT_ARCH_32 1\n");
    } else {
        features.push_str("#define GIT_ARCH_64 1\n");
    }

    if ssh {
        if let Some(path) = env::var_os("DEP_SSH2_INCLUDE") {
            cfg.include(path);
        }
        features.push_str("#define GIT_SSH 1\n");
        features.push_str("#define GIT_SSH_LIBSSH2 1\n");
        features.push_str("#define GIT_SSH_LIBSSH2_MEMORY_CREDENTIALS 1\n");
    }
    if https {
        features.push_str("#define GIT_HTTPS 1\n");

        if windows {
            features.push_str("#define GIT_WINHTTP 1\n");
        } else if target.contains("apple") {
            features.push_str("#define GIT_SECURE_TRANSPORT 1\n");
        } else {
            features.push_str("#define GIT_OPENSSL 1\n");
            if let Some(path) = env::var_os("DEP_OPENSSL_INCLUDE") {
                cfg.include(path);
            }
        }
    }

    // Use the CollisionDetection SHA1 implementation.
    features.push_str("#define GIT_SHA1_COLLISIONDETECT 1\n");
    // cfg.define("SHA1DC_NO_STANDARD_INCLUDES", "1");
    // cfg.define("SHA1DC_CUSTOM_INCLUDE_SHA1_C", "\"common.h\"");
    // cfg.define("SHA1DC_CUSTOM_INCLUDE_UBC_CHECK_C", "\"common.h\"");
    cfg.file("libgit2/src/util/hash/collisiondetect.c");
    cfg.file("libgit2/src/util/hash/sha1dc/sha1.c");
    cfg.file("libgit2/src/util/hash/sha1dc/ubc_check.c");

    if https {
        if windows {
            features.push_str("#define GIT_SHA256_WIN32 1\n");
            cfg.file("libgit2/src/util/hash/win32.c");
        } else if target.contains("apple") {
            features.push_str("#define GIT_SHA256_COMMON_CRYPTO 1\n");
            cfg.file("libgit2/src/util/hash/common_crypto.c");
        } else {
            features.push_str("#define GIT_SHA256_OPENSSL 1\n");
            cfg.file("libgit2/src/util/hash/openssl.c");
        }
    } else {
        features.push_str("#define GIT_SHA1_BUILTIN 1\n");
        features.push_str("#define GIT_SHA256_BUILTIN 1\n");
        cfg.file("libgit2/src/util/hash/builtin.c");
        cfg.file("libgit2/src/util/hash/rfc6234/sha224-256.c");
    }

    if let Some(path) = env::var_os("DEP_Z_INCLUDE") {
        cfg.include(path);
    }

    if target.contains("apple") {
        features.push_str("#define GIT_USE_ICONV 1\n");
    }

    features.push_str("#endif\n");
    fs::write(include.join("git2_features.h"), features).unwrap();

    cfg.compile("git2");

    println!("cargo:root={}", dst.display());

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=winhttp");
        println!("cargo:rustc-link-lib=rpcrt4");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=crypt32");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=advapi32");
    }

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    println!("cargo:rerun-if-changed=libgit2/include");
    println!("cargo:rerun-if-changed=libgit2/src");
    println!("cargo:rerun-if-changed=libgit2/deps");
}

fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    for e in from.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let from = e.path();
        let to = to.as_ref().join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            fs::create_dir_all(&to).unwrap();
            cp_r(&from, &to);
        } else {
            println!("{} => {}", from.display(), to.display());
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    let path = path.as_ref();
    if !path.exists() {
        panic!("Path {} does not exist", path.display());
    }
    // sort the C files to ensure a deterministic build for reproducible builds
    let dir = path.read_dir().unwrap();
    let mut paths = dir.collect::<io::Result<Vec<_>>>().unwrap();
    paths.sort_by_key(|e| e.path());

    for e in paths {
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            // skip dirs for now
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}
