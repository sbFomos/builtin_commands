#![no_std]

extern crate libc;
use libc::{c_char, c_int, printf};

pub fn space() -> &'static str {
    "\n\0"
}

pub fn print_main(arg: &str) {
    unsafe { printf(arg.as_ptr() as *const c_char); }
    unsafe { printf(space().as_ptr() as *const c_char); }
}
