extern crate julia_sys;

use std::thread::spawn;
use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;

use julia_sys::*;

fn main() {
    unsafe {
        let s = CString::new(julia_home().to_string_lossy().as_ref()).unwrap();

        jl_init(s.as_ptr());

        let mut v = vec![];
        let s = CString::new(r#"println("shit")"#).unwrap();
        jl_eval_string(s.as_ptr());
        for i in 0..2 {
            let handle = spawn(move || {
                let s = CString::new(r#"println("shit")"#).unwrap();
                jl_eval_string(s.as_ptr());
            });
            v.push(handle);
        }
        println!("reach");
    }
    sleep(Duration::from_secs(10));
}
