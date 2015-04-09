extern crate csg;
extern crate libc;

#[allow(non_camel_case_types)]

use csg::curses::{Curses};

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println!("usage: csg <file>");
        -1
    }
    let filename : &str = args[1].as_slice();
    let curses = Curses::new(filename);
    curses.init();
    curses.run_forever();
}