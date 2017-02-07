
include!("init.rs");

#[test]
fn eval_works() {
    init();
    unsafe {
        let s = cs!(r#"println("it works")"#);
        let r = jl_eval_string(s.into_raw());
        assert!(jl_is_nothing(r));
        jl_exit(0);
    }
}