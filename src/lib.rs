#![allow(unstable)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(missing_copy_implementations)]
#![allow(non_upper_case_globals)]
#![allow(raw_pointer_derive)]
#[macro_use]
extern crate lazy_static;

pub mod csgui;
pub mod sqlite;
pub mod curses;
pub mod cext;
pub mod osext;