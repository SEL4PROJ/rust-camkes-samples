# Rust camkes samples

Rust camkes samples
=======================

A collection of camkes apps, libs and configs for using rust on self.

libbtreemap
-----------
The rust btreemap collection implementation exported as a static lib that can be used by other c libraries or apps.  Uses rusty-cheddar to generate c header files.

hellorust
---------
A hello world camkes component that uses rust to print hello world

keyvalue
--------
A more complex camkes app that uses 3 components to store values in a rust btreemap data structure.
Demonstrates using:
* Importing a rust library from c: libbtreemap is a rust implementation exported as a static library that is linked against a c camkes app.
* A camkes component written in rust
* Rust and C camkes components communicating using rpc, notification and dataport interfaces.
* [rust-bindgen](https://github.com/crabtw/rust-bindgen): For generating rust bindings for camkes template functions
* [rusty-cheddar](https://github.com/Sean1708/rusty-cheddar): For generating c header files from rust source files
* [compiler-rt](http://compiler-rt.llvm.org/): A clang library that provides some builtins that rust needs.



License
========

The files in this repository are release under standard open source licenses.
Please see individual file headers and the `LICENSE_BSD2`.txt file for details.
