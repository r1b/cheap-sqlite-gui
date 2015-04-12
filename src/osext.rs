extern crate core;

use std::os::{getenv};
use self::core::str::FromStr;

// XXX: Yes I made this I am very proud of it ^_^
pub fn get_env_as<T: FromStr>(s : &str) -> Option<T> {
    match getenv(s) {
        Some(n) => n.parse::<T>(),
        None => None
    }
}