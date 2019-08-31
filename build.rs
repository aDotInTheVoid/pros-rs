use cbindgen;
use std::env;

fn rs_to_c() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", crate_dir);
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bin/bindings.h");
}

fn main(){
    rs_to_c();
}