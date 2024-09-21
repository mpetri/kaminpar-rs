fn main() {
    use cmake::Config;

    let dst = Config::new("vendor/KaMinPar")
        .define("KAMINPAR_64BIT_EDGE_IDS", "ON")
        .define("KAMINPAR_64BIT_EDGE_WEIGHTS", "ON")
        .no_build_target(true)
        .build();
    let path = format!("{}/build/kaminpar-shm/", dst.display());

    println!("Building vendor KaMinPar finished successfully");

    cxx_build::bridge("src/lib.rs")
        .file("src/kaminpar_wrapper.cc")
        .flag_if_supported("-std=c++20")
        .define("KAMINPAR_64BIT_EDGE_IDS", "ON")
        .define("KAMINPAR_64BIT_EDGE_WEIGHTS", "ON")
        .static_flag(true)
        .compile("kaminpar-rs");

    println!("Bridge compiled successfully");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/kaminpar_wrapper.cc");
    println!("cargo:rerun-if-changed=include/kaminpar_wrapper.h");
    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static=kaminpar_shm");
    println!("cargo:rustc-link-lib=dylib=tbb");
    println!("cargo:rustc-link-lib=dylib=tbbmalloc");
}
