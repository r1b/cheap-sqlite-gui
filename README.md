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

+ q 	-> Quit/previous screen
+ hjkl 	-> Movement
+ e 	-> Edit entry

## Organization

+ src/cext.rs		-> Utility functions for converting repr from C <-> Rust
+ src/curses.rs 	-> GUI routines & Curses FFI
+ src/lib.rs 		-> Module structure
+ src/main.rs 		-> Entry point
+ src/sqlite.rs 	-> Sqlite FFI

## TODO

+ Stability (e.g limit entry width)
+ Scroll on rows/columns that exceed screen height/width
+ Cell editing
+ Pull out Screen struct from curses.rs & add wrapper struct (separate view from controller)
+ Better "highlighting" of selected cell