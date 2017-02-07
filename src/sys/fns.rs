use libc::{c_char, c_int, c_void, uintptr_t};
use std::mem::{transmute, size_of};
use super::*;

// c macros
pub unsafe fn jl_astaggedvalue(value: *mut jl_value_t) -> *mut jl_taggedvalue_t {
    let newv: *mut c_char = transmute(value);
    let ptr = newv.wrapping_offset(-(size_of::<jl_taggedvalue_t>() as isize));
    let result: *mut jl_taggedvalue_t = transmute(ptr);
    result
}

pub unsafe fn jl_typeof(value: *mut jl_value_t) -> *mut jl_value_t {
    let header = (*jl_astaggedvalue(value)).header;
    let lhs: *mut jl_value_t = transmute(header & !(15 as uintptr_t));
    lhs
}

pub unsafe fn jl_valueof(value: *mut jl_value_t) -> *mut jl_value_t {
    let valptr: *mut c_char = transmute(value);
    transmute(valptr.wrapping_offset(size_of::<jl_taggedvalue_t>() as isize))
}

pub unsafe fn jl_typeis(value: *mut jl_value_t, typ: *mut jl_datatype_t) -> bool {
    let typ: *mut jl_value_t = transmute(typ);
    let val = jl_typeof(value);
    val == typ
}

pub unsafe fn jl_is_float(value: *mut jl_value_t) -> bool {
    jl_subtype(value, jl_floatingpoint_type as *mut c_void, 1) == 1
}

pub unsafe fn jl_is_int64(value: *mut jl_value_t) -> bool {
    jl_typeis(value, jl_int64_type)
}

pub unsafe fn jl_is_int32(value: *mut jl_value_t) -> bool {
    jl_typeis(value, jl_int32_type)
}

pub unsafe fn jl_is_int16(value: *mut jl_value_t) -> bool {
    jl_typeis(value, jl_int16_type)
}

pub unsafe fn jl_is_int8(value: *mut jl_value_t) -> bool {
    jl_typeis(value, jl_int8_type)
}

pub unsafe fn jl_is_nothing(value: *mut jl_value_t) -> bool {
    let lhs: *mut jl_value_t = transmute(value);
    let rhs: *mut jl_value_t = transmute(jl_nothing);
    lhs == rhs
}

// init
extern "C" {
    pub fn jl_init(julia_home_dir: *const c_char);
    pub fn jl_init_with_image(julia_home_dir: *const c_char, image_relative_path: *const c_char);
    pub fn jl_is_initialized() -> c_int;
}

// run
extern "C" {
    pub fn jl_eval_string(str: *const c_char) -> *mut jl_value_t;
}

// exit
extern "C" {
    pub fn jl_atexit_hook(status: c_int);
    pub fn jl_exit(status: c_int);
}

// value
extern "C" {
    pub fn jl_box_bool(x: i8) -> *mut jl_value_t;
    pub fn jl_box_int8(x: i8) -> *mut jl_value_t;
    pub fn jl_box_uint8(x: u8) -> *mut jl_value_t;
    pub fn jl_box_int16(x: i16) -> *mut jl_value_t;
    pub fn jl_box_uint16(x: u16) -> *mut jl_value_t;
    pub fn jl_box_int32(x: i32) -> *mut jl_value_t;
    pub fn jl_box_uint32(x: u32) -> *mut jl_value_t;
    pub fn jl_box_char(x: u32) -> *mut jl_value_t;
    pub fn jl_box_int64(x: i64) -> *mut jl_value_t;
    pub fn jl_box_uint64(x: u64) -> *mut jl_value_t;
    pub fn jl_box_float32(x: f32) -> *mut jl_value_t;
    pub fn jl_box_float64(x: f64) -> *mut jl_value_t;
    pub fn jl_box_voidpointer(x: *mut c_void) -> *mut jl_value_t;
    pub fn jl_box_ssavalue(x: usize) -> *mut jl_value_t;
    pub fn jl_box_slotnumber(x: usize) -> *mut jl_value_t;
    pub fn jl_box8(t: *mut jl_datatype_t, x: i8) -> *mut jl_value_t;
    pub fn jl_box16(t: *mut jl_datatype_t, x: i16) -> *mut jl_value_t;
    pub fn jl_box32(t: *mut jl_datatype_t, x: i32) -> *mut jl_value_t;
    pub fn jl_box64(t: *mut jl_datatype_t, x: i64) -> *mut jl_value_t;

    pub fn jl_unbox_bool(v: *mut jl_value_t) -> i8;
    pub fn jl_unbox_int8(v: *mut jl_value_t) -> i8;
    pub fn jl_unbox_uint8(v: *mut jl_value_t) -> u8;
    pub fn jl_unbox_int16(v: *mut jl_value_t) -> i16;
    pub fn jl_unbox_uint16(v: *mut jl_value_t) -> u16;
    pub fn jl_unbox_int32(v: *mut jl_value_t) -> i32;
    pub fn jl_unbox_uint32(v: *mut jl_value_t) -> u32;
    pub fn jl_unbox_int64(v: *mut jl_value_t) -> i64;
    pub fn jl_unbox_uint64(v: *mut jl_value_t) -> u64;
    pub fn jl_unbox_float32(v: *mut jl_value_t) -> f32;
    pub fn jl_unbox_float64(v: *mut jl_value_t) -> f64;
    pub fn jl_unbox_voidpointer(v: *mut jl_value_t) -> *mut c_void;

    pub fn jl_subtype(a: *mut jl_value_t, b: *mut jl_value_t, ta: c_int) -> c_int;
}
