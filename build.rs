extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let cur_dir = PathBuf::from(env::current_dir().unwrap());

    let lib_path = cur_dir.join("build");
    println!("cargo:rustc-link-search={}", lib_path.display());

    let header_path = cur_dir
        .join("src")
        .join("example.h")
        .into_os_string()
        .into_string()
        .expect("Couldn't convert header path to string");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    println!("cargo:rerun-if-changed={}", header_path);
    let builder = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    builder
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path)
        .expect("Failed to write bindings");
    println!("cargo:rustc-link-lib={}", "example");
}
