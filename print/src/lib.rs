#![no_std]

extern crate fk_std;
use fk_std::{c_char, {libc::printf}};

pub fn space() -> &'static str {
    "\n\0"
}

pub fn print_main(arg: &str) {
    unsafe { printf(arg.as_ptr() as *const c_char); }
    unsafe { printf(space().as_ptr() as *const c_char); }
}
