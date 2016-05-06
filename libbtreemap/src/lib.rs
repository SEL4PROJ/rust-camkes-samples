// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//

extern crate libc;

use std::slice;
use std::collections::BTreeMap;

use libc::{uintptr_t, uint32_t};


// This section defines c-like structs

#[repr(C)]
pub struct CBTreeMap(BTreeMap<uint32_t, uint32_t>);

#[repr(C)]
pub enum Option_C {
    None,
    Some,
}

#[repr(C)]
pub struct OptionVec_culong {
    option_type: Option_C,
    len: uintptr_t,
    max_size: uintptr_t,
    elements: *mut uint32_t,
}

#[repr(C)]
pub struct OptionValue_culong {
    option_type: Option_C,
    val: uint32_t,
}

// Create new btreemap and return mutable pointer
// Box is used to ensure that the pointer returned is from the heap and won't be
// cleaned up once the function returns.
#[no_mangle]
pub extern "C" fn btreemap_new() -> *mut CBTreeMap {
    Box::into_raw(Box::new(CBTreeMap(BTreeMap::new())))
}

// The following functions rely on the caller passing in a valid raw pointer to an
// initialised BTreeMap.  This is then dereferenced to perform the operations on it which
// is unsafe behaviour hence the need for unsafe blocks.
//


#[no_mangle]
pub extern "C" fn btreemap_contains_key(btreemap: *mut CBTreeMap, key: uint32_t) -> bool {
    unsafe { (*btreemap).0.contains_key(&key) }
}


#[no_mangle]
pub extern "C" fn btreemap_insert(btreemap: *mut CBTreeMap,
                                  key: uint32_t,
                                  value: uint32_t,
                                  res: *mut OptionValue_culong)
                                  -> uint32_t {
    let &mut OptionValue_culong { ref mut option_type, ref mut val } = unsafe { &mut *res };
    println!("key {:?} value {:?} ", key, value);

    match unsafe { (*btreemap).0.insert(key, value) } {
        Some(old) => {
            *option_type = Option_C::Some;
            *val = old;
        }
        None => {
            *option_type = Option_C::None;
        }
    };
    0
}

#[no_mangle]
pub extern "C" fn btreemap_keys(btreemap: *mut CBTreeMap, res: *mut OptionVec_culong) -> uint32_t {
    let &mut OptionVec_culong {
         ref mut option_type,
         ref mut len,
         ref max_size,
         ref mut elements
    } = unsafe{&mut *res};
    match unsafe { (*btreemap).0.keys() } {
        keys => {
            // Currently if there are more keys than the size of the buffer, we return error
            *len = keys.len();
            if *len > *max_size {
                return 1;
            }

            // Copy the result into the provided buffer
            *option_type = Option_C::Some;
            let a: &mut [uint32_t] = unsafe { slice::from_raw_parts_mut(*elements, *max_size) };
            for (&x, p) in keys.zip(a) {
                *p = x;
            }
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn btreemap_values(btreemap: *mut CBTreeMap,
                                  res: *mut OptionVec_culong)
                                  -> uint32_t {
    let &mut OptionVec_culong {
         ref mut option_type,
         ref mut len,
         ref max_size,
         ref mut elements
    } = unsafe{&mut *res};
    match unsafe { (*btreemap).0.values() } {
        vec => {
            // Currently if there are more keys than the size of the buffer, we return error
            *len = vec.len();
            if *len > *max_size {
                return 1;
            }

            // Copy the result into the provided buffer
            *option_type = Option_C::Some;
            let a: &mut [uint32_t] = unsafe { slice::from_raw_parts_mut(*elements, *max_size) };
            for (&x, p) in vec.zip(a) {
                *p = x;
            }
            0
        }
    }
}


#[no_mangle]
pub extern "C" fn btreemap_remove(btreemap: *mut CBTreeMap,
                                  key: uint32_t,
                                  res: *mut OptionValue_culong)
                                  -> uint32_t {
    let &mut OptionValue_culong { ref mut option_type, ref mut val } = unsafe { &mut *res };
    match unsafe { (*btreemap).0.remove(&key) } {
        Some(old) => {
            *option_type = Option_C::Some;
            *val = old;
        }
        None => {
            *option_type = Option_C::None;
        }
    };
    0
}

#[no_mangle]
pub extern "C" fn btreemap_len(btreemap: *mut CBTreeMap) -> uintptr_t {
    unsafe { (*btreemap).0.len() as uintptr_t }
}


#[no_mangle]
pub extern "C" fn btreemap_is_empty(btreemap: *mut CBTreeMap) -> bool {
    unsafe { (*btreemap).0.is_empty() }
}
