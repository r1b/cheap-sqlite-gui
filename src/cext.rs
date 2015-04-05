/// Helpers for converting data representations Rust <---> C

extern crate libc;

use cext::libc::{c_char, c_int, c_void};
use std::ffi::{CString, c_str_to_bytes};
use std::mem;
use std::raw::Slice;
use std::str;

/** Converts a vector of owned rust strings to a vector of c strings */
pub fn strs_to_cstrs(strs : Vec<&str>) -> Vec<CString> {
  let result : Vec<CString> = strs.into_iter().map(|x : &str| { CString::from_vec(x.as_bytes().to_vec()) }).collect();
  result
}

/** Converts an array of c strings to a vector of owned rust strings */
pub fn cstrs_to_strs(cstrs : *const *const c_char, len : usize) -> Vec<&'static str> {
  let mut result : Vec<&str> = Vec::new();
  let cstrs: &[*const c_char] = unsafe { mem::transmute(Slice { data: cstrs, len: len }) };
  for cstr in cstrs.iter() {
    unsafe { result.push(str::from_utf8(c_str_to_bytes(cstr)).ok().unwrap()) }
  }
  result
}