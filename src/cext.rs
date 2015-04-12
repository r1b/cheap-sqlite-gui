/// Helpers for converting data representations Rust <---> C

extern crate libc;

use cext::libc::{c_char, c_int, c_void};
use std::ffi::{CString, c_str_to_bytes};
use std::mem;
use std::raw::Slice;
use std::str;

/// C Constants
pub static TRUE : c_int = 1;
pub static FALSE : c_int = 0;

/// Converts a rust string to a c string
pub fn str_to_cstr(s : &str) -> CString {
    CString::from_slice(s.as_bytes()) 
}

/// Converts a c string to an owned rust string
pub fn cstr_to_str(cs : *const c_char) -> String {
    str::from_utf8(unsafe { c_str_to_bytes(&cs) }).ok().unwrap().to_string()
}

/// Converts a vector of rust strings to a vector of c strings
pub fn strs_to_cstrs(strs : Vec<&str>) -> Vec<CString> {
    let result : Vec<CString> = strs.into_iter().map(|s : &str| { 
        str_to_cstr(s)
    }).collect();
    result
}

/// Converts an array of c strings to a vector of owned rust strings
pub fn cstrs_to_strs(cstrs : *const *const c_char, len : usize) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    let cstrs: &[*const c_char] = unsafe { mem::transmute(Slice { data: cstrs, len: len }) };
    for cstr in cstrs.iter() {
        let rstr = cstr_to_str(*cstr);
        result.push(rstr);
    }
    result
}