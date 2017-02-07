extern crate julia_sys;

use julia_sys::*;

use std::sync::{Once, ONCE_INIT};

#[warn(dead_code)]
static INIT_JULIA_ONCE: Once = ONCE_INIT;

#[allow(dead_code)]
fn init() {
    INIT_JULIA_ONCE.call_once(|| {
        let julia_home = julia_home();
        println!("julia home is {:?}", julia_home);
        unsafe {
            jl_init(julia_home.to_string_lossy().as_ptr() as *mut _);
        }
        println!("initialization of julia succeeded")
    });

}