
include!("init.rs");

#[test]
fn unbox_f64() {
    init();
    unsafe {
        let s = cs!(r#"54.321"#);
        let r = jl_eval_string(s.into_raw());
        assert!(jl_is_float(r));
        assert_eq!(jl_unbox_float64(r), 54.321);
        jl_exit(0);
    }
}