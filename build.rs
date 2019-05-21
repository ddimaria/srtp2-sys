
use std::env;
use std::process::Command;

fn main() {
    let crate_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = &env::var("OUT_DIR").unwrap();
    let target = &env::var("TARGET").unwrap();

    if target.contains("msvc") {
        panic!("libsrtp doesn't support windows toolchain")
    }

    bindgen::Builder::default()
        .whitelist_function("srtp_.*")
        .clang_args(vec!["-I.", "-I./libsrtp/include", "-I./libsrtp/crypto/include"])
        .header("libsrtp/include/srtp_priv.h")
        .generate()
        .expect("Failed to generate libsrtp binding")
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Failed to write libsrtp binding");

    println!("cargo:rerun-if-changed=libsrtp");

    let mut configure = Command::new(format!("{}/libsrtp/configure", crate_dir));

    if cfg!(feature = "enable-debug-logging") {
        configure.arg("--enable-debug-logging");
    }
    if cfg!(feature = "enable-log-stdout") {
        configure.arg("--enable-log-stdout");
    }

    let out = configure
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `./configure` on libsrtp");
    assert!(out.status.success(), "Failed to execute `./configure` on libsrtp");

    let out = make_cmd::make()
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `make` on libsrtp");
    assert!(out.status.success(), "Failed to execute `make` on libsrtp");

    println!("cargo:rustc-link-lib=static=srtp2");
    println!("cargo:rustc-link-search={}", out_dir);
}
