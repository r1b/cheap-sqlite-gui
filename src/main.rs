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
    let result : Result<(), String> = {
	    let mut csgui = match CSG::new(args[1].as_slice()) {
	    	Ok(csgui) => { csgui },
	    	Err(msg) => { 
	    		println!("{}", msg);
	    		return;
	    	}
	    };

    	csgui.run_forever()
    };

    match result {
    	Ok(_) => { },
    	Err(msg) => { 
    		println!("{}", msg);
    		return;
    	}
    }
}