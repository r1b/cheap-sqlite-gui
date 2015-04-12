/// Sqlite3 FFI wrapper
extern crate libc;

use sqlite::libc::{c_char, c_int, c_void};
use std::ptr;
use std::sync::{Mutex};
use cext::{cstrs_to_strs, str_to_cstr};

static LIST_TABLES_QUERY : &'static str = "select name from sqlite_master where type = 'table';";
// XXX: Apparently format strings have to be literals?
// static DUMP_TABLE_QUERY : &'static str = "select * from {}";

// Lazy initialization of global execution state, thx Jesse
// It's thread-safe too, which doesn't matter in the current
// iteration of this software but may matter later
lazy_static! {
    pub static ref exec_results: Mutex<ExecResult> = Mutex::new(ExecResult::new());
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
}

/// Callback method used for exec
extern fn exec_cb(_ : *const c_void, 
                  num_cols : c_int, 
                  col_text : *const *const c_char, 
                  col_names : *const *const c_char) -> c_int {
    let num_cols = num_cols as usize;
    let col_text = cstrs_to_strs(col_text, num_cols);
    let col_names = cstrs_to_strs(col_names, num_cols);
    let mut results = exec_results.lock().unwrap();

    let need_num_cols = match results.num_cols {
        Some(_) => false,
        None => true
    };
    if need_num_cols {
        results.num_cols = Some(num_cols);
    }

    let need_col_names = match results.col_names {
        Some(_) => false,
        None => true
    };
    if need_col_names {
        results.col_names = Some(col_names);
    }

    results.col_text.push(col_text);

    0 as c_int
}

/// Represents the result of a call to SQLite library
pub struct ExecResult {
    /// Number of columns
    pub num_cols : Option<usize>,
    /// Text in the columns
    pub col_text : Vec<Vec<String>>,
    /// Names of the columns
    pub col_names : Option<Vec<String>>
}

impl ExecResult {
    pub fn new() -> ExecResult {
        ExecResult {
            num_cols : None,
            col_text : Vec::new(),
            col_names : None
        }
    }

    /// Clears an execution result
    pub fn reset(&mut self) {
        self.num_cols = None;
        self.col_text = Vec::new();
        self.col_names = None;
    }

    /// Return an immutable reference to column names
    pub fn get_col_names(&self) -> Option<&Vec<String>> {
        self.col_names.as_ref()
    }
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
        Sqlite::open(filename, &db_handle).ok();
        Sqlite { db_handle: db_handle }
    }

    /// Open a new database
    pub fn open(filename : &str, db_handle : & *const c_sqlite3) -> Result<(), String> {
        let filename = str_to_cstr(filename);
        let ret = unsafe { sqlite3_open(filename.as_ptr(), db_handle as *const *const c_sqlite3) };
        match ret {
            0 => { Ok(()) }
            _ => { Err(format!("sqlite3_open: got error code: {}", ret)) }
        }
    }

    /// Execute a command from the SQLite code
    pub fn exec(&self,
                sql: &str,
                cb: extern fn(*const c_void, c_int, *const *const c_char, *const *const c_char) -> c_int,
                ) -> i32 {
        let sql = str_to_cstr(sql);
        exec_results.lock().unwrap().reset();
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