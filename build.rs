// Copyright (c) 2019, Nixon Enraght-Moony
// All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen;
use cbindgen;

const H_FILE_OUT: &str = "bin/bindings.h";
const RS_FILE_OUT: &str = "prg.rs"; // PROS RAW GENERATED.
const PROS_OMNIBUS: &str = "pros_omnibus.h";

const CC: &str = "arm-none-eabi-gcc";

// Create an h file that has bindings for the rust fns
fn rs_to_c() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(H_FILE_OUT);
}

fn c_to_rs() {
    // Somewhere in target
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Because bindgen uses sys clang, and their is as best I can tell no way
    // to change it, we need to manualy preprocess the code with arm-none-eabi-gcc
    Command::new(CC)
        // CC keeps coments for docs
        // -dD also adds #defines
        // https://gcc.gnu.org/onlinedocs/gcc/Preprocessor-Options.html
        .args("-CC -dD".split_ascii_whitespace())
        // Get PROS headers
        .args("-I ./include/".split_ascii_whitespace())
        // Input PROS core api
        .args("-E include/api.h".split_ascii_whitespace())
        // Output file spec
        .arg("-o")
        .arg(out_path.join(PROS_OMNIBUS))
        // Run the command
        .output()
        .expect("failed to preprocess code");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("::cortex_a9_types")
        .use_core()
        .rustfmt_bindings(true)
        .generate_comments(true)
        .header(
            out_path
                .join(PROS_OMNIBUS)
                .as_path()
                .to_str()
                .expect("Weird UTF8 Error, check your filesystem"),
        )
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join(RS_FILE_OUT))
        .expect("Couldn't write bindings");
}

fn main() {
    rs_to_c();
    c_to_rs();
}
