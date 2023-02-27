use cmake::Config;
use std::env;
use std::path::PathBuf;

extern crate bindgen;

fn main() {

    let iowa_dir = env::var("IOWA_DIR").expect("environment variable IOWA_DIR is not set");
    let iowa_dir_path = PathBuf::from(iowa_dir).canonicalize().expect("cannot canonicalize path");
    let iowa_include_dir_path = iowa_dir_path.join("include");
    let iowa_include_dir_path_str  = iowa_include_dir_path.to_str().expect("IOWA include dir path is not a valid string");
    dbg!(iowa_include_dir_path_str);

    let mut arg = String::from("-I");
    arg.push_str(iowa_include_dir_path_str);

    // println!("cargo:warning=arg is {}", arg);

    let iowa_dot_h_path = iowa_include_dir_path.join("iowa.h");
    let iowa_dot_h_path_str = iowa_dot_h_path.to_str().expect("IOWA main header file path is not a valid string");

    println!("cargo:warning=IOWA main header file path is {}", iowa_dot_h_path_str);


    let bindings = bindgen::Builder::default()
    .header(iowa_dot_h_path_str)
    .generate()
    .expect("cannot generate bindings");
   
    // println!("cargo:warning={:?}", bindings);

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let out_path = PathBuf::from("./src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    let dst = Config::new("src/iowa")
    .build_target("iowa")
    .build();

    println!("cargo:warning={}", dst.display());
    println!("cargo:rustc-link-search={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=iowa");
}
