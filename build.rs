extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pkg = pkg_config::Config::new()
        .atleast_version("0.3.0")
        .probe("wireplumber-0.3")
        .unwrap();

    let mut clang_args: Vec<String> = Vec::new();
    for item in pkg.include_paths.iter() {
        let mut include: String = "-I".to_owned();
        include.push_str(&*item.display().to_string());
        clang_args.push(include.parse().unwrap());
    }

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-lib=wireplumber-0.3");

    let header = pkg.include_paths[0].join("wp/wp.h").display().to_string();

    println!("cargo:rerun-if-changed={}", &header);

    let bindings = bindgen::Builder::default()
        .header(&header)
        .clang_args(clang_args.iter())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}