use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/include");
    // println!("cargo:rustc-link-lib=lldb");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-x", "c++", "--std=c++14", "-fkeep-inline-functions"])
        .opaque_type("std::.*")
        // .clang_arg("-x c++")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
