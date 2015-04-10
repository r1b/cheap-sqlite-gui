#![allow(unstable)]
extern crate csg;
extern crate libc;

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