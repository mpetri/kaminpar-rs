fn main() {
    use cmake::Config;
    use std::path::{Path, PathBuf};

    fn read_cmake_cache_var(build_dir: &Path, key: &str) -> Option<String> {
        let cache_path = build_dir.join("CMakeCache.txt");
        let cache = std::fs::read_to_string(cache_path).ok()?;
        for line in cache.lines() {
            // Format: KEY:TYPE=value
            if let Some(rest) = line.strip_prefix(key) {
                if let Some(value) = rest.splitn(2, '=').nth(1) {
                    return Some(value.trim().to_string());
                }
            }
        }
        None
    }

    fn find_tbb_include_dir(build_dir: &Path) -> Option<PathBuf> {
        if let Some(include) = std::env::var_os("TBB_INCLUDE_DIR").map(PathBuf::from) {
            if include.join("tbb/global_control.h").exists() {
                return Some(include);
            }
        }

        // If KaMinPar fetched oneTBB via FetchContent, headers usually land here.
        let fetched = build_dir
            .join("_deps")
            .join("tbb-src")
            .join("include");
        if fetched.join("tbb/global_control.h").exists() {
            return Some(fetched);
        }

        // If CMake found TBB via its config, derive an install prefix from `TBB_DIR`.
        if let Some(tbb_dir) = read_cmake_cache_var(build_dir, "TBB_DIR:PATH") {
            let mut cursor = PathBuf::from(tbb_dir);
            for _ in 0..8 {
                let candidate = cursor.join("include");
                if candidate.join("tbb/global_control.h").exists() {
                    return Some(candidate);
                }
                if !cursor.pop() {
                    break;
                }
            }
        }

        // Fallbacks for common install prefixes.
        for prefix in ["/usr", "/usr/local", "/opt/homebrew", "/opt/local"] {
            let candidate = Path::new(prefix).join("include");
            if candidate.join("tbb/global_control.h").exists() {
                return Some(candidate);
            }
        }

        None
    }

    fn find_tbb_lib_dir(build_dir: &Path) -> Option<PathBuf> {
        if let Some(lib) = std::env::var_os("TBB_LIB_DIR").map(PathBuf::from) {
            return Some(lib);
        }

        // If CMake found TBB via its config, `TBB_DIR` usually lives under the lib dir.
        if let Some(tbb_dir) = read_cmake_cache_var(build_dir, "TBB_DIR:PATH") {
            let tbb_dir = PathBuf::from(tbb_dir);
            if let Some(lib_dir) = tbb_dir.parent().and_then(|p| p.parent()) {
                return Some(lib_dir.to_path_buf());
            }
        }

        // If KaMinPar fetched oneTBB, the built artifacts can be in these places depending on CMake.
        for candidate in [
            build_dir.join("_deps").join("tbb-build"),
            build_dir.join("_deps").join("tbb-build").join("lib"),
            build_dir.join("_deps").join("tbb-build").join("lib64"),
        ] {
            if candidate.exists() {
                return Some(candidate);
            }
        }

        None
    }

    let dst = Config::new("vendor/KaMinPar")
        .define("KAMINPAR_64BIT_EDGE_IDS", "ON")
        .define("KAMINPAR_ENABLE_TBB_MALLOC", "OFF")
        .no_build_target(true)
        .build();
    let build_dir = dst.join("build");
    let path = dst.join("build").join("kaminpar-shm");

    let tbb_include_dir = find_tbb_include_dir(&build_dir).unwrap_or_else(|| {
        panic!(
            "Could not find TBB headers (missing `tbb/global_control.h`). Install oneTBB (package often named `tbb`) or set `TBB_INCLUDE_DIR`."
        )
    });
    if let Some(tbb_lib_dir) = find_tbb_lib_dir(&build_dir) {
        println!("cargo:rustc-link-search=native={}", tbb_lib_dir.display());
        println!("cargo:rerun-if-env-changed=TBB_LIB_DIR");
    }
    println!("cargo:rerun-if-env-changed=TBB_INCLUDE_DIR");

    println!("Building vendor KaMinPar finished successfully");

    cxx_build::bridge("src/lib.rs")
        .file("src/kaminpar_wrapper.cc")
        .flag_if_supported("-std=c++20")
        .include(&tbb_include_dir)
        .include("vendor/KaMinPar/include")
        .define("KAMINPAR_64BIT_EDGE_IDS", "1")
        .compile("kaminpar-rs");

    println!("Bridge compiled successfully");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/kaminpar_wrapper.cc");
    println!("cargo:rerun-if-changed=include/kaminpar_wrapper.h");
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=KaMinPar");
    println!("cargo:rustc-link-lib=dylib=tbb");
}
