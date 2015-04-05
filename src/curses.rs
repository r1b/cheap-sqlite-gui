/// Curses FFI wrapper

extern crate libc;

use sqlite::libc::c_char;
use std::ffi::{CString, c_str_to_bytes};
use std::mem;

#[link(name="ncurses")]
extern {
    fn curses_version() -> *const c_char; 
}

pub fn csg_curses_version() -> String {
    let raw_version : *const i8 = unsafe { curses_version() };
    let buf = unsafe { c_str_to_bytes(mem::transmute(&raw_version)) };

    return String::from_utf8(buf.to_vec()).unwrap();
}
