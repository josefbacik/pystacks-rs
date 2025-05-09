#[cfg(feature = "bindgen-source")]
fn generate_bindings() {
    use std::path::PathBuf;

    use bindgen::builder;
    use pkg_config;

    let out_dir =
        PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR must be set in build script"));

    pkg_config::probe_library("fmt").unwrap_or_else(|_| panic!("Failed to find fmt library"));
    pkg_config::probe_library("re2").unwrap_or_else(|_| panic!("Failed to find re2 library"));
    pkg_config::probe_library("libcap").unwrap_or_else(|_| panic!("Failed to find libcap library"));

    println!("cargo:rerun-if-changed=strobelight-libs/strobelight/bpf_lib");
    println!("cargo:rustc-link-search={}", out_dir.display());

    let include_arg = format!("-I{}", out_dir.display());
    let status = std::process::Command::new("make")
        .env("INSTALL_DIR", &out_dir)
        .env("EXTRA_CFLAGS", &include_arg)
        .arg("-C")
        .arg("strobelight-libs/strobelight/bpf_lib/python")
        .arg("install")
        .status()
        .expect("Failed to run make");

    assert!(status.success(), "Make command failed");

    let bindings = builder()
        .header("strobelight-libs/strobelight/bpf_lib/python/pystacks/pystacks.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args([include_arg, "-x".to_string(), "c++".to_string()])
        .allowlist_function("pystacks_.*")
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "bindgen-source"))]
fn generate_bindings() {}

fn main() {
    generate_bindings();
}
