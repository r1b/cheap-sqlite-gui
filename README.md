# cheap-sqlite-gui

## Authors

+ Robert C Jensen <rcj@ccs.neu.edu>
+ Howard Cheung <howardc@ccs.neu.edu>

## Build on OS X

+ `brew install sqlite3`
+ `brew install ncurses`
+ `cargo build`

## Running

`cargo run test.db`

## Usage

+ q 	  -> Quit/previous screen
+ hjkl 	-> Movement
+ e 	  -> Edit entry

## Organization

+ src/cext.rs		  -> Utility functions for converting repr from C <-> Rust
+ src/csgui.rs    -> GUI logic
+ src/curses.rs 	-> Curses FFI
+ src/lib.rs 		  -> Module structure
+ src/main.rs 		-> Entry point
+ src/osext.rs		-> Utility functions for dealing with the environment
+ src/sqlite.rs 	-> Sqlite FFI

## TODO

+ Stability (e.g limit entry width)
+ Scroll on rows/columns that exceed screen height/width
+ Cell editing
+ Better "highlighting" of selected cell
