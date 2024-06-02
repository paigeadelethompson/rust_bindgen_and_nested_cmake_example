use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let out = Config::new("libhello/")
        .build();

    //panic!("{}",env::current_dir().unwrap().display());
    //panic!("{}", out.display());

    println!("cargo:rustc-link-search={}/lib", out.display());
    println!("cargo:rustc-link-lib=static=hello");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/proto.h", out.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}