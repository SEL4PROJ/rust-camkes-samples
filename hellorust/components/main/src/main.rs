// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//

// This is the camkes entry point for this app
#[no_mangle]
pub extern "C" fn run() -> isize {
    println!("Hello, world!!");
    0
}
