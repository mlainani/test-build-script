#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));

fn main() {
    unsafe {
        let mut _iowa_ctx = iowa_init(std::ptr::null_mut());
    }
}
