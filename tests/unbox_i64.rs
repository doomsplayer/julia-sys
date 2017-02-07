
include!("init.rs");

#[test]
fn unbox_i64() {
    init();
    unsafe {
        let r = jl_eval_string(r#"12345"#.as_ptr() as *mut _);
        assert!(jl_is_int64(r));
        assert_eq!(jl_unbox_int64(r), 12345);
        jl_exit(0);
    }
}
