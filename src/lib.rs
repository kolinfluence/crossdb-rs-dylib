#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

// Make the bindings public so they can be used in examples
pub use self::bindings::*;

// Include the bindings in a separate module
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CString;

fn main() {
    unsafe {
        let c = CString::new(":memory:").unwrap();
        let status = xdb_open(c.as_ptr());
        dbg!(status);
    }
}
