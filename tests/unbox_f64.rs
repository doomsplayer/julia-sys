
include!("init.rs");

#[test]
fn unbox_f64() {
    init();
    unsafe {
        let r = jl_eval_string(r#"54.321"#.as_ptr() as *mut _);
        assert!(jl_is_float(r));
        assert_eq!(jl_unbox_float64(r), 54.321);
        jl_exit(0);
    }
}