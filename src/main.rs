extern crate csg;

use csg::sqlite::csg_sqlite3_libversion;
use csg::curses::csg_curses_version;

fn main() {
   let sqlite_version = csg_sqlite3_libversion(); 
   let curses_version = csg_curses_version();
   println!("Sqlite version: {}", sqlite_version);
   println!("Curses version: {}", curses_version);
}
