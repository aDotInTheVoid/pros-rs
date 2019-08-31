//! Raw ffi bindings to pros
//!
//! Items here all directly call pros functions

// Because c stuff doesn't use rust style, and without it cargo emits 
// so many warnings it crashes travis.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/prg.rs"));
