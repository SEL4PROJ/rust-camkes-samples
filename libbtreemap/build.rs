// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//


extern crate cheddar;

fn main() {
    // This generates a header file named btreemap.h in include/generated which then
    // allows c modules that depend on this library to use the exported functions
    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .run_build("include/generated/btreemap.h");
}
