extern crate csg;
extern crate libc;

#[allow(non_camel_case_types)]

use csg::curses::{Curses};

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println!("usage: csg <file>");
        return;
    }
    let mut curses = Curses::new(args[1].as_slice());
    curses.run_forever();
}