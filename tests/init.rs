extern crate julia_sys;

use julia_sys::*;

use std::sync::{Once, ONCE_INIT};
use std::ffi::CString;

#[warn(dead_code)]
static INIT_JULIA_ONCE: Once = ONCE_INIT;

macro_rules! cs {
    ($v: expr) => (
        CString::new($v).unwrap();
    )
}

#[allow(dead_code)]
fn init() {
    INIT_JULIA_ONCE.call_once(|| {
        let julia_home = julia_home();
        println!("julia home is {:?}", julia_home);
        unsafe {
            let cs = cs!(julia_home.to_string_lossy().as_ref());

            jl_init(cs.into_raw());
        }
        println!("initialization of julia succeeded")
    });

}