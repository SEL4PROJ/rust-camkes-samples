// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//


extern crate bindgen;

use bindgen::{Bindings, BindgenOptions};
use std::default::Default;
use std::fs;
use std::env;
use std::path::{PathBuf, Path};

/**
 * This build.rs script is run before the rust source is compiled and we use it to
 * generate rust bindings of the camkes.h symbols so that we can call them easier.
 *
 * The generated file can be found in (Must compile at least once):
 * 	 target/{target}/{debug|release}/build/{library or binary name}/out/generated.rs
 *
 * (note: The camkes.h file that gets used can be found at:
 * 	build/{arm/imx31|or another target}/keyvalue/include/main_object/generated/camkes.h)
 */
fn main() {

    // Setup build and stage paths from global env variables provided by kbuild
    let build_dir = PathBuf::from(&env::var("BUILD_DIR").expect("BUILD_DIR env var"));
    let stage_dir = PathBuf::from(&env::var("STAGE_DIR").expect("STAGE_DIR env var"));

    // Construct bindgen options
    // see: https://github.com/crabtw/rust-bindgen for config options
    let mut bindgen_opts = BindgenOptions::default();
    // Add the staging include directory to resolve #include files
    bindgen_opts.clang_args.push(format!("-I{}", stage_dir.join("include").display()));

    // Add camkes.h file that we generate bindings for
    bindgen_opts.clang_args.push(format!("{}",
                                         build_dir.join("include/main_object/generated/camkes.h")
                                                  .display()));
    // Generate bindings for builtins
    bindgen_opts.builtins = true;

    // Generate bindings
    let bindings: Bindings = Bindings::generate(&bindgen_opts, None, None)
                                 .expect("Generating bindings");

    // Save bindings to generated.rs file in
    // target/{target}/{debug|release}/build/{library or binary name}/out/ directory.
    // This file is then imported into the rust source at compile time.
    let gen = fs::File::create(&Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var"))
                                    .join("generated.rs"))
                  .expect("Create file");
    bindings.write(Box::new(gen)).expect("Writing bindings to file")
}
