/// Sqlite3 FFI wrapper
extern crate libc;

use sqlite::libc::{c_char, c_int, c_void};
use std::ffi::{CString, c_str_to_bytes};
use std::mem;
use std::ptr;
use std::sync::{Mutex};
use cext;

static LIST_TABLES_QUERY : &'static str = "select name from sqlite_master where type = 'table';";
// XXX: Apparently format strings have to be literals?
// static DUMP_TABLE_QUERY : &'static str = "select * from {}";

lazy_static! {
    pub static ref exec_results: Mutex<Vec<ExecResult>> = Mutex::new(Vec::new());
}

pub static MAX_TABLE_NAME_LENGTH : usize = 128;

/** Wrapper for opaque struct */
#[repr(C)]
pub struct c_sqlite3;

#[link(name="sqlite3")]
extern {
    fn sqlite3_open(filename: *const c_char,        
                    db_handle: *const *const c_sqlite3) -> c_int;
    fn sqlite3_exec(db_handle: *const c_sqlite3, 
                    sql: *const c_char,
                    cb: extern fn(*const c_void, 
                                  c_int, 
                                  *const *const c_char, 
                                  *const *const c_char) -> c_int,
                    arg: *const c_void,
                    errmsg: *const *const c_char) -> c_int;
    fn sqlite3_libversion() -> *const c_char;
}

/// Callback method used for exec
extern fn exec_cb(db_handle : *const c_void, 
                         num_cols : c_int, 
                         col_text : *const *const c_char, 
                         col_names : *const *const c_char) -> c_int {
    let col_text = cext::cstrs_to_strs(col_text, num_cols as usize);
    let col_names = cext::cstrs_to_strs(col_names, num_cols as usize);

    
    let result = ExecResult {
        num_cols : num_cols as usize,
        col_text : col_text,
        col_names : col_names
    };

    let mut results = exec_results.lock().unwrap();
    results.push(result);

    0 as c_int
}

/// Represents the result of a call to SQLite library
pub struct ExecResult {
    /// Number of columns
    pub num_cols : usize,
    /// Text in the columns
    pub col_text : Vec<String>,
    /// Names of the columns
    pub col_names : Vec<String>
}

#[derive(Clone)]
pub struct Sqlite {
    /// A pointer to SQLite databse
    db_handle : *const c_sqlite3
}

impl Sqlite {  
    /// Constructor
    pub fn new(filename : &str) -> Sqlite {
        let db_handle = ptr::null();
        let res = Sqlite::open(filename, &db_handle).ok();
        Sqlite { db_handle: db_handle }
    }

    /// Open a new database
    pub fn open(filename : &str, db_handle : & *const c_sqlite3) -> Result<(), String> {
        let filename = CString::from_slice(filename.as_bytes());
        let ret = unsafe { sqlite3_open(filename.as_ptr(), db_handle as *const *const c_sqlite3) };
        match ret {
            0 => { Ok(()) }
            _ => { Err(format!("sqlite3_open: got error code: {}", ret)) }
        }
    }

    /// Get the current version
    pub fn version() -> String {
        let raw_version : *const i8 = unsafe { sqlite3_libversion() };
        let buf = unsafe { c_str_to_bytes(mem::transmute(&raw_version)) };

        String::from_utf8(buf.to_vec()).unwrap()
    }

    /// Execute a command from the SQLite code
    pub fn exec(&self,
                sql: &str,
                cb: extern fn(*const c_void, c_int, *const *const c_char, *const *const c_char) -> c_int,
                ) -> i32 {
        let sql = CString::from_slice(sql.as_bytes());
        exec_results.lock().unwrap().clear();
        unsafe { 
            sqlite3_exec(self.db_handle, 
                         sql.as_ptr(), 
                         cb,
                         self.db_handle as *const c_void,
                         ptr::null()) // Ignoring errmsg! Otherwise we have to free it
        } 
    }

    /// Calls SQLite to list all tables
    pub fn list_tables(&self) {
        self.exec(LIST_TABLES_QUERY, exec_cb);
    }

    /// Dumps all table entries
    pub fn dump_table(&self, table : String) {
        self.exec(format!("select * from {};", table).as_slice(), exec_cb);
    }
}