use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=crossdb");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::builder()
        .header("crossdb/include/crossdb.h")
        .allowlist_function("xdb_.*")
        .allowlist_function("XDB_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .write_to_file(out_path.join("./bindings.rs"))
        .unwrap();
    println!("cargo:rerun-if-changed=crossdb/");
}
