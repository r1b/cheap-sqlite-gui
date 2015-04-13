#![allow(unstable)]
#![allow(dead_code)]
extern crate csg;
extern crate libc;

use csg::csgui::{CSG};

/// Entry point
fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println!("usage: csg <file>");
        return;
    }
    let mut csgui = CSG::new(args[1].as_slice());
    csgui.run_forever();
}