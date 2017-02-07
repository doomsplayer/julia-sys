
include!("init.rs");

#[test]
fn eval_works() {
    init();
    unsafe {
        let r = jl_eval_string(r#"println("it works")"#.as_ptr() as *mut _);
        assert!(jl_is_nothing(r));
        jl_exit(0);
    }
}
