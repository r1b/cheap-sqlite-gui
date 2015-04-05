/// Sqlite3 FFI wrapper

extern crate libc;

use sqlite::libc::{c_char, c_int, c_void};
use std::ffi::{CString, c_str_to_bytes};
use std::mem;
use std::ptr;
use cext;

/** Wrapper for opaque struct */
pub struct csg_sqlite3;

#[link(name="sqlite3")]
extern {
    fn sqlite3_libversion() -> *const c_char;
    fn sqlite3_open(filename: *const c_char, 		
  		              ppDb: *const *const csg_sqlite3) -> c_int;
    fn sqlite3_exec(db_handle: *const csg_sqlite3, 
                    sql: *const c_char,
                    cb: extern fn(*const c_void, c_int, *const *const c_char, *const *const c_char) -> c_int,
                    arg: *const c_void,
                    errmsg: *const *const c_char) -> c_int;
}

/** Get sqlite3 library version */
pub fn csg_sqlite3_libversion() -> String {
    let raw_version : *const i8 = unsafe { sqlite3_libversion() };
    let buf = unsafe { c_str_to_bytes(mem::transmute(&raw_version)) };

    return String::from_utf8(buf.to_vec()).unwrap();
}

/** Opens a sqlite3 database, storing a database handle in ppDb */
pub fn csg_sqlite3_open(filename : &str, db_handle : & *const csg_sqlite3) -> i32 {
	let filename = CString::from_slice(filename.as_bytes());
	unsafe { sqlite3_open(filename.as_ptr(), db_handle as *const *const csg_sqlite3) }
}

/** Executes a sql query, runs cb for each result row */
pub fn csg_sqlite3_exec(db_handle: *const csg_sqlite3, 
                           sql: &str,
                           cb: extern fn(*const c_void, c_int, *const *const c_char, *const *const c_char) -> c_int,
                           first_arg: *const csg_sqlite3,
                           errmsg: Vec<&str>) -> i32 {
    let sql = CString::from_slice(sql.as_bytes());
    let errmsg : Vec<CString> = cext::strs_to_cstrs(errmsg);
    unsafe { sqlite3_exec(db_handle, 
                          sql.as_ptr(), 
                          cb,
                          first_arg as *const c_void,
                          ptr::null())} // Ignoring errmsg! Otherwise we have to free it
}