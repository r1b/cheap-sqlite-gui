extern crate csg;
extern crate libc;

use csg::sqlite::{Sqlite, exec_results};
use csg::cext;
use csg::curses;
use std::ptr;
use libc::{c_char, c_int, c_void};

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        panic!("usage: csg <file>");
    }
    let filename : &str = args[1].as_slice();
    let sqlite = Sqlite::new(filename);
    sqlite.list_tables();
    //initscr();
    let results = exec_results.lock().unwrap();
    for result in (*results).iter() {
        for text in result.col_text.iter() {
            println!("{}", text);
        }
    }
}