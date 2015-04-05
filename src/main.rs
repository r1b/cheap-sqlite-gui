extern crate csg;
extern crate libc;

use csg::sqlite;
use csg::cext;
use std::ptr;
use libc::{c_char, c_int, c_void};

static LIST_TABLES_QUERY : &'static str = "select name from sqlite_master where type = 'table';";

extern fn list_tables_cb(db_handle : *const c_void, 
	  num_cols : c_int, 
	  col_text : *const *const c_char, 
	  col_names : *const *const c_char) -> c_int {
	let col_text = cext::cstrs_to_strs(col_text, num_cols as usize);
	for text in col_text.iter() {
		println!("{}", text);
	}
	0 as c_int
}

fn main() {
	let args = std::os::args();
	if args.len() < 2 {
		panic!("usage: csg <file>");
	}
	let filename : &str = args[1].as_slice();
	let db_handle = ptr::null();
   	match db_connect(filename, &db_handle) {
   		Ok(_) => {
   			let errmsg : Vec<&str> = Vec::new();
   			let res : c_int = sqlite::csg_sqlite3_exec(db_handle, 
   										  LIST_TABLES_QUERY, 
   										  list_tables_cb,
   										  db_handle,
   										  errmsg);
   			println!("heyyy");
   		}
   		Err(e) => {
   			panic!("{}", e);
   		}
   	};
}

fn db_connect(filename : &str, db_handle : &*const sqlite::csg_sqlite3) -> Result<(), String> {
	let ret = sqlite::csg_sqlite3_open(filename, db_handle);
	match ret {
		0 => { Ok(()) }
		_ => { Err(format!("sqlite3_open: got error code: {}", ret)) }
	}
}