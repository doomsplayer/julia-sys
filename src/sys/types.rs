use libc::{c_ulong, c_void};

// types
#[allow(non_camel_case_types)]
pub type jl_value_t = c_void;

#[allow(non_camel_case_types)]
type jl_tupletype_t = jl_datatype_t;

#[allow(non_camel_case_types)]
pub type jl_fptr_t = Option<unsafe extern "C" fn(arg1: *mut jl_value_t,
                                                 arg2: *mut *mut jl_value_t,
                                                 arg3: u32)
                                                 -> *mut jl_value_t>;

// structs
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub union jl_taggedvalue_t {
    pub header: usize,
    pub next: *mut jl_taggedvalue_t,
    pub type_: *mut jl_value_t,
    pub bits: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_typemap_level_t {
    pub arg1: jl_ordereddict_t,
    pub targ: jl_ordereddict_t,
    pub linear: *mut jl_typemap_entry_t,
    pub any: jl_typemap_t,
    pub key: *mut jl_value_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub union jl_typemap_t {
    pub node: *mut jl_typemap_level_t,
    pub leaf: *mut jl_typemap_entry_t,
    pub unknown: *mut jl_value_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_typemap_entry_t {
    pub next: *mut jl_typemap_entry_t,
    pub sig: *mut jl_tupletype_t,
    pub tvars: *mut jl_svec_t,
    pub simplesig: *mut jl_tupletype_t,
    pub guardsigs: *mut jl_svec_t,
    pub func: jl_typemap_t__jl_typemap_entry_t__bindgen_ty_1,
    pub isleafsig: i8,
    pub issimplesig: i8,
    pub va: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub union jl_typemap_t__jl_typemap_entry_t__bindgen_ty_1 {
    pub value: *mut jl_value_t,
    pub linfo: *mut jl_lambda_info_t,
    pub method: *mut jl_method_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_lambda_info_t {
    pub rettype: *mut jl_value_t,
    pub sparam_syms: *mut jl_svec_t,
    pub sparam_vals: *mut jl_svec_t,
    pub specTypes: *mut jl_tupletype_t,
    pub code: *mut jl_value_t,
    pub slottypes: *mut jl_value_t,
    pub ssavaluetypes: *mut jl_value_t,
    pub slotnames: *mut jl_array_t,
    pub slotflags: *mut jl_array_t,
    pub unspecialized_ducttape: *mut jl_lambda_info_t,
    pub def: *mut jl_method_t,
    pub constval: *mut jl_value_t,
    pub nargs: i32,
    pub isva: i8,
    pub inferred: i8,
    pub pure_: i8,
    pub inlineable: i8,
    pub inInference: i8,
    pub inCompile: i8,
    pub jlcall_api: i8,
    pub compile_traced: i8,
    pub fptr: jl_fptr_t,
    pub functionObjectsDecls: jl_llvm_functions_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_method_t {
    pub name: *mut jl_sym_t,
    pub module: *mut jl_module_t,
    pub file: *mut jl_sym_t,
    pub line: i32,
    pub sig: *mut jl_tupletype_t,
    pub tvars: *mut jl_svec_t,
    pub ambig: *mut jl_value_t,
    pub specializations: jl_typemap_t,
    pub lambda_template: *mut jl_lambda_info_t,
    pub roots: *mut jl_array_t,
    pub invokes: jl_typemap_t,
    pub called: i32,
    pub isstaged: i8,
    pub needs_sparam_vals_ducttape: u8,
    pub traced: u8,
    pub writelock: jl_mutex_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_sym_t {
    pub left: *mut jl_sym_t,
    pub right: *mut jl_sym_t,
    pub hash: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_svec_t {
    pub length: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_typename_t {
    pub name: *mut jl_sym_t,
    pub module: *mut jl_module_t,
    pub names: *mut jl_svec_t,
    pub primary: *mut jl_value_t,
    pub cache: *mut jl_svec_t,
    pub linearcache: *mut jl_svec_t,
    pub hash: isize,
    pub mt: *mut jl_methtable_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_module_t {
    pub name: *mut jl_sym_t,
    pub parent: *mut jl_module_t,
    pub bindings: htable_t,
    pub usings: arraylist_t,
    pub istopmod: u8,
    pub uuid: u64,
    pub counter: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_methtable_t {
    pub name: *mut jl_sym_t,
    pub defs: jl_typemap_t,
    pub cache: jl_typemap_t,
    pub max_args: isize,
    pub kwsorter: *mut jl_value_t,
    pub module: *mut jl_module_t,
    pub writelock: jl_mutex_t,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct htable_t {
    pub size: usize,
    pub table: *mut *mut c_void,
    pub _space: [*mut c_void; 32],
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct arraylist_t {
    pub len: usize,
    pub max: usize,
    pub items: *mut *mut c_void,
    pub _space: [*mut c_void; 29usize],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_ordereddict_t {
    pub indexes: *mut jl_array_t,
    pub values: *mut jl_array_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_datatype_t {
    pub name: *mut jl_typename_t,
    pub super_: *mut jl_datatype_t,
    pub parameters: *mut jl_svec_t,
    pub types: *mut jl_svec_t,
    pub instance: *mut jl_value_t,
    pub layout: *const jl_datatype_layout_t,
    pub size: i32,
    pub ninitialized: i32,
    pub uid: u32,
    pub abstract_: u8,
    pub mutabl: u8,
    pub struct_decl: *mut c_void,
    pub ditype: *mut c_void,
    pub depth: i32,
    pub hastypevars: i8,
    pub haswildcard: i8,
    pub isleaftype: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_array_t {
    pub data: *mut c_void,
    pub length: usize,
    pub flags: jl_array_flags_t,
    pub elsize: u16,
    pub offset: u32,
    pub nrows: usize,
    pub __bindgen_anon_1: _bindgen_ty_84__bindgen_ty_1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_llvm_functions_t {
    pub functionObject: *mut c_void,
    pub specFunctionObject: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_mutex_t {
    pub owner: c_ulong,
    pub count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_datatype_layout_t {
    pub nfields: u32,
    pub _bitfield_1: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct jl_array_flags_t {
    pub _bitfield_1: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub union _bindgen_ty_84__bindgen_ty_1 {
    pub maxsize: usize,
    pub ncols: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct jl_tvar_t {
    pub name: *mut jl_sym_t,
    pub lb: *mut jl_value_t,
    pub ub: *mut jl_value_t,
    pub bound: u8,
}