use super::*;

// constants
extern "C" {
    #[link_name = "jl_any_type"]
    pub static mut jl_any_type: *mut jl_datatype_t;

    #[link_name = "jl_type_type"]
    pub static mut jl_type_type: *mut jl_datatype_t;

    #[link_name = "jl_typetype_tvar"]
    pub static mut jl_typetype_tvar: *mut jl_tvar_t;

    #[link_name = "jl_typetype_type"]
    pub static mut jl_typetype_type: *mut jl_datatype_t;

    #[link_name = "jl_ANY_flag"]
    pub static mut jl_ANY_flag: *mut jl_value_t;

    #[link_name = "jl_typename_type"]
    pub static mut jl_typename_type: *mut jl_datatype_t;

    #[link_name = "jl_typector_type"]
    pub static mut jl_typector_type: *mut jl_datatype_t;

    #[link_name = "jl_sym_type"]
    pub static mut jl_sym_type: *mut jl_datatype_t;

    #[link_name = "jl_symbol_type"]
    pub static mut jl_symbol_type: *mut jl_datatype_t;

    #[link_name = "jl_ssavalue_type"]
    pub static mut jl_ssavalue_type: *mut jl_datatype_t;

    #[link_name = "jl_abstractslot_type"]
    pub static mut jl_abstractslot_type: *mut jl_datatype_t;

    #[link_name = "jl_slotnumber_type"]
    pub static mut jl_slotnumber_type: *mut jl_datatype_t;

    #[link_name = "jl_typedslot_type"]
    pub static mut jl_typedslot_type: *mut jl_datatype_t;

    #[link_name = "jl_simplevector_type"]
    pub static mut jl_simplevector_type: *mut jl_datatype_t;

    #[link_name = "jl_tuple_typename"]
    pub static mut jl_tuple_typename: *mut jl_typename_t;

    #[link_name = "jl_vecelement_typename"]
    pub static mut jl_vecelement_typename: *mut jl_typename_t;

    #[link_name = "jl_anytuple_type"]
    pub static mut jl_anytuple_type: *mut jl_datatype_t;

    #[link_name = "jl_anytuple_type_type"]
    pub static mut jl_anytuple_type_type: *mut jl_datatype_t;

    #[link_name = "jl_vararg_type"]
    pub static mut jl_vararg_type: *mut jl_datatype_t;

    #[link_name = "jl_tvar_type"]
    pub static mut jl_tvar_type: *mut jl_datatype_t;

    #[link_name = "jl_task_type"]
    pub static mut jl_task_type: *mut jl_datatype_t;

    #[link_name = "jl_function_type"]
    pub static mut jl_function_type: *mut jl_datatype_t;

    #[link_name = "jl_builtin_type"]
    pub static mut jl_builtin_type: *mut jl_datatype_t;

    #[link_name = "jl_uniontype_type"]
    pub static mut jl_uniontype_type: *mut jl_datatype_t;

    #[link_name = "jl_datatype_type"]
    pub static mut jl_datatype_type: *mut jl_datatype_t;

    #[link_name = "jl_bottom_type"]
    pub static mut jl_bottom_type: *mut jl_value_t;

    #[link_name = "jl_lambda_info_type"]
    pub static mut jl_lambda_info_type: *mut jl_datatype_t;

    #[link_name = "jl_method_type"]
    pub static mut jl_method_type: *mut jl_datatype_t;

    #[link_name = "jl_module_type"]
    pub static mut jl_module_type: *mut jl_datatype_t;

    #[link_name = "jl_abstractarray_type"]
    pub static mut jl_abstractarray_type: *mut jl_datatype_t;

    #[link_name = "jl_densearray_type"]
    pub static mut jl_densearray_type: *mut jl_datatype_t;

    #[link_name = "jl_array_type"]
    pub static mut jl_array_type: *mut jl_datatype_t;

    #[link_name = "jl_array_typename"]
    pub static mut jl_array_typename: *mut jl_typename_t;

    #[link_name = "jl_weakref_type"]
    pub static mut jl_weakref_type: *mut jl_datatype_t;

    #[link_name = "jl_string_type"]
    pub static mut jl_string_type: *mut jl_datatype_t;

    #[link_name = "jl_errorexception_type"]
    pub static mut jl_errorexception_type: *mut jl_datatype_t;

    #[link_name = "jl_argumenterror_type"]
    pub static mut jl_argumenterror_type: *mut jl_datatype_t;

    #[link_name = "jl_loaderror_type"]
    pub static mut jl_loaderror_type: *mut jl_datatype_t;

    #[link_name = "jl_initerror_type"]
    pub static mut jl_initerror_type: *mut jl_datatype_t;

    #[link_name = "jl_typeerror_type"]
    pub static mut jl_typeerror_type: *mut jl_datatype_t;

    #[link_name = "jl_methoderror_type"]
    pub static mut jl_methoderror_type: *mut jl_datatype_t;

    #[link_name = "jl_undefvarerror_type"]
    pub static mut jl_undefvarerror_type: *mut jl_datatype_t;

    #[link_name = "jl_stackovf_exception"]
    pub static mut jl_stackovf_exception: *mut jl_value_t;

    #[link_name = "jl_memory_exception"]
    pub static mut jl_memory_exception: *mut jl_value_t;

    #[link_name = "jl_readonlymemory_exception"]
    pub static mut jl_readonlymemory_exception: *mut jl_value_t;

    #[link_name = "jl_diverror_exception"]
    pub static mut jl_diverror_exception: *mut jl_value_t;

    #[link_name = "jl_domain_exception"]
    pub static mut jl_domain_exception: *mut jl_value_t;

    #[link_name = "jl_overflow_exception"]
    pub static mut jl_overflow_exception: *mut jl_value_t;

    #[link_name = "jl_inexact_exception"]
    pub static mut jl_inexact_exception: *mut jl_value_t;

    #[link_name = "jl_undefref_exception"]
    pub static mut jl_undefref_exception: *mut jl_value_t;

    #[link_name = "jl_interrupt_exception"]
    pub static mut jl_interrupt_exception: *mut jl_value_t;

    #[link_name = "jl_boundserror_type"]
    pub static mut jl_boundserror_type: *mut jl_datatype_t;

    #[link_name = "jl_an_empty_vec_any"]
    pub static mut jl_an_empty_vec_any: *mut jl_value_t;

    #[link_name = "jl_bool_type"]
    pub static mut jl_bool_type: *mut jl_datatype_t;

    #[link_name = "jl_char_type"]
    pub static mut jl_char_type: *mut jl_datatype_t;

    #[link_name = "jl_int8_type"]
    pub static mut jl_int8_type: *mut jl_datatype_t;

    #[link_name = "jl_uint8_type"]
    pub static mut jl_uint8_type: *mut jl_datatype_t;

    #[link_name = "jl_int16_type"]
    pub static mut jl_int16_type: *mut jl_datatype_t;

    #[link_name = "jl_uint16_type"]
    pub static mut jl_uint16_type: *mut jl_datatype_t;

    #[link_name = "jl_int32_type"]
    pub static mut jl_int32_type: *mut jl_datatype_t;

    #[link_name = "jl_uint32_type"]
    pub static mut jl_uint32_type: *mut jl_datatype_t;

    #[link_name = "jl_int64_type"]
    pub static mut jl_int64_type: *mut jl_datatype_t;

    #[link_name = "jl_uint64_type"]
    pub static mut jl_uint64_type: *mut jl_datatype_t;

    #[link_name = "jl_float16_type"]
    pub static mut jl_float16_type: *mut jl_datatype_t;

    #[link_name = "jl_float32_type"]
    pub static mut jl_float32_type: *mut jl_datatype_t;

    #[link_name = "jl_float64_type"]
    pub static mut jl_float64_type: *mut jl_datatype_t;

    #[link_name = "jl_floatingpoint_type"]
    pub static mut jl_floatingpoint_type: *mut jl_datatype_t;

    #[link_name = "jl_number_type"]
    pub static mut jl_number_type: *mut jl_datatype_t;

    #[link_name = "jl_void_type"]
    pub static mut jl_void_type: *mut jl_datatype_t;

    #[link_name = "jl_complex_type"]
    pub static mut jl_complex_type: *mut jl_datatype_t;

    #[link_name = "jl_signed_type"]
    pub static mut jl_signed_type: *mut jl_datatype_t;

    #[link_name = "jl_voidpointer_type"]
    pub static mut jl_voidpointer_type: *mut jl_datatype_t;

    #[link_name = "jl_pointer_type"]
    pub static mut jl_pointer_type: *mut jl_datatype_t;

    #[link_name = "jl_ref_type"]
    pub static mut jl_ref_type: *mut jl_datatype_t;

    #[link_name = "jl_array_uint8_type"]
    pub static mut jl_array_uint8_type: *mut jl_value_t;

    #[link_name = "jl_array_any_type"]
    pub static mut jl_array_any_type: *mut jl_value_t;

    #[link_name = "jl_array_symbol_type"]
    pub static mut jl_array_symbol_type: *mut jl_value_t;

    #[link_name = "jl_expr_type"]
    pub static mut jl_expr_type: *mut jl_datatype_t;

    #[link_name = "jl_globalref_type"]
    pub static mut jl_globalref_type: *mut jl_datatype_t;

    #[link_name = "jl_linenumbernode_type"]
    pub static mut jl_linenumbernode_type: *mut jl_datatype_t;

    #[link_name = "jl_labelnode_type"]
    pub static mut jl_labelnode_type: *mut jl_datatype_t;

    #[link_name = "jl_gotonode_type"]
    pub static mut jl_gotonode_type: *mut jl_datatype_t;

    #[link_name = "jl_quotenode_type"]
    pub static mut jl_quotenode_type: *mut jl_datatype_t;

    #[link_name = "jl_newvarnode_type"]
    pub static mut jl_newvarnode_type: *mut jl_datatype_t;

    #[link_name = "jl_intrinsic_type"]
    pub static mut jl_intrinsic_type: *mut jl_datatype_t;

    #[link_name = "jl_methtable_type"]
    pub static mut jl_methtable_type: *mut jl_datatype_t;

    #[link_name = "jl_typemap_level_type"]
    pub static mut jl_typemap_level_type: *mut jl_datatype_t;

    #[link_name = "jl_typemap_entry_type"]
    pub static mut jl_typemap_entry_type: *mut jl_datatype_t;

    #[link_name = "jl_emptysvec"]
    pub static mut jl_emptysvec: *mut jl_svec_t;

    #[link_name = "jl_emptytuple"]
    pub static mut jl_emptytuple: *mut jl_value_t;

    #[link_name = "jl_true"]
    pub static mut jl_true: *mut jl_value_t;

    #[link_name = "jl_false"]
    pub static mut jl_false: *mut jl_value_t;

    #[link_name = "jl_nothing"]
    pub static mut jl_nothing: *mut jl_value_t;

    #[link_name = "call_sym"]
    pub static mut call_sym: *mut jl_sym_t;

    #[link_name = "invoke_sym"]
    pub static mut invoke_sym: *mut jl_sym_t;

    #[link_name = "empty_sym"]
    pub static mut empty_sym: *mut jl_sym_t;

    #[link_name = "body_sym"]
    pub static mut body_sym: *mut jl_sym_t;

    #[link_name = "dots_sym"]
    pub static mut dots_sym: *mut jl_sym_t;

    #[link_name = "vararg_sym"]
    pub static mut vararg_sym: *mut jl_sym_t;

    #[link_name = "quote_sym"]
    pub static mut quote_sym: *mut jl_sym_t;

    #[link_name = "newvar_sym"]
    pub static mut newvar_sym: *mut jl_sym_t;

    #[link_name = "top_sym"]
    pub static mut top_sym: *mut jl_sym_t;

    #[link_name = "dot_sym"]
    pub static mut dot_sym: *mut jl_sym_t;

    #[link_name = "line_sym"]
    pub static mut line_sym: *mut jl_sym_t;

    #[link_name = "toplevel_sym"]
    pub static mut toplevel_sym: *mut jl_sym_t;

    #[link_name = "core_sym"]
    pub static mut core_sym: *mut jl_sym_t;

    #[link_name = "globalref_sym"]
    pub static mut globalref_sym: *mut jl_sym_t;

    #[link_name = "jl_incomplete_sym"]
    pub static mut jl_incomplete_sym: *mut jl_sym_t;

    #[link_name = "error_sym"]
    pub static mut error_sym: *mut jl_sym_t;

    #[link_name = "amp_sym"]
    pub static mut amp_sym: *mut jl_sym_t;

    #[link_name = "module_sym"]
    pub static mut module_sym: *mut jl_sym_t;

    #[link_name = "colons_sym"]
    pub static mut colons_sym: *mut jl_sym_t;

    #[link_name = "export_sym"]
    pub static mut export_sym: *mut jl_sym_t;

    #[link_name = "import_sym"]
    pub static mut import_sym: *mut jl_sym_t;

    #[link_name = "importall_sym"]
    pub static mut importall_sym: *mut jl_sym_t;

    #[link_name = "using_sym"]
    pub static mut using_sym: *mut jl_sym_t;

    #[link_name = "goto_sym"]
    pub static mut goto_sym: *mut jl_sym_t;

    #[link_name = "goto_ifnot_sym"]
    pub static mut goto_ifnot_sym: *mut jl_sym_t;

    #[link_name = "label_sym"]
    pub static mut label_sym: *mut jl_sym_t;

    #[link_name = "return_sym"]
    pub static mut return_sym: *mut jl_sym_t;

    #[link_name = "lambda_sym"]
    pub static mut lambda_sym: *mut jl_sym_t;

    #[link_name = "assign_sym"]
    pub static mut assign_sym: *mut jl_sym_t;

    #[link_name = "method_sym"]
    pub static mut method_sym: *mut jl_sym_t;

    #[link_name = "slot_sym"]
    pub static mut slot_sym: *mut jl_sym_t;

    #[link_name = "enter_sym"]
    pub static mut enter_sym: *mut jl_sym_t;

    #[link_name = "leave_sym"]
    pub static mut leave_sym: *mut jl_sym_t;

    #[link_name = "exc_sym"]
    pub static mut exc_sym: *mut jl_sym_t;

    #[link_name = "new_sym"]
    pub static mut new_sym: *mut jl_sym_t;

    #[link_name = "compiler_temp_sym"]
    pub static mut compiler_temp_sym: *mut jl_sym_t;

    #[link_name = "const_sym"]
    pub static mut const_sym: *mut jl_sym_t;

    #[link_name = "thunk_sym"]
    pub static mut thunk_sym: *mut jl_sym_t;

    #[link_name = "anonymous_sym"]
    pub static mut anonymous_sym: *mut jl_sym_t;

    #[link_name = "underscore_sym"]
    pub static mut underscore_sym: *mut jl_sym_t;

    #[link_name = "abstracttype_sym"]
    pub static mut abstracttype_sym: *mut jl_sym_t;

    #[link_name = "bitstype_sym"]
    pub static mut bitstype_sym: *mut jl_sym_t;

    #[link_name = "compositetype_sym"]
    pub static mut compositetype_sym: *mut jl_sym_t;

    #[link_name = "global_sym"]
    pub static mut global_sym: *mut jl_sym_t;

    #[link_name = "unused_sym"]
    pub static mut unused_sym: *mut jl_sym_t;

    #[link_name = "boundscheck_sym"]
    pub static mut boundscheck_sym: *mut jl_sym_t;

    #[link_name = "inbounds_sym"]
    pub static mut inbounds_sym: *mut jl_sym_t;

    #[link_name = "copyast_sym"]
    pub static mut copyast_sym: *mut jl_sym_t;

    #[link_name = "fastmath_sym"]
    pub static mut fastmath_sym: *mut jl_sym_t;

    #[link_name = "pure_sym"]
    pub static mut pure_sym: *mut jl_sym_t;

    #[link_name = "simdloop_sym"]
    pub static mut simdloop_sym: *mut jl_sym_t;

    #[link_name = "meta_sym"]
    pub static mut meta_sym: *mut jl_sym_t;

    #[link_name = "list_sym"]
    pub static mut list_sym: *mut jl_sym_t;

    #[link_name = "inert_sym"]
    pub static mut inert_sym: *mut jl_sym_t;

    #[link_name = "static_parameter_sym"]
    pub static mut static_parameter_sym: *mut jl_sym_t;

    #[link_name = "polly_sym"]
    pub static mut polly_sym: *mut jl_sym_t;

    #[link_name = "inline_sym"]
    pub static mut inline_sym: *mut jl_sym_t;
}