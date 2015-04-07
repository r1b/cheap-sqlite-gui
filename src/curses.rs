/// Curses FFI wrapper

extern crate libc;

use sqlite::libc::c_char;
use std::ffi::{CString, c_str_to_bytes};
use std::mem;

//#[allow(non_camel_case_types)]
//pub struct WINDOW;

#[link(name="ncurses")]
extern {
    fn curses_version() -> *const c_char; 
    fn initscr(); // -> WINDOW;
}

pub fn csg_initscr() {
    unsafe { initscr(); }
}

pub fn csg_curses_version() -> String {
    let raw_version : *const i8 = unsafe { curses_version() };
    let buf = unsafe { c_str_to_bytes(mem::transmute(&raw_version)) };

    return String::from_utf8(buf.to_vec()).unwrap();
}
