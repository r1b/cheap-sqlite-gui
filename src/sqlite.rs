extern crate libc;

use sqlite::libc::c_char;
use std::ffi::{CString, c_str_to_bytes};
use std::mem;

#[link(name="sqlite3")]
extern {
    fn sqlite3_libversion() -> *const c_char; 
}

pub fn csg_sqlite3_libversion() -> String {
    let raw_version : *const i8 = unsafe { sqlite3_libversion() };
    let buf = unsafe { c_str_to_bytes(mem::transmute(&raw_version)) };

    return String::from_utf8(buf.to_vec()).unwrap();
}
