#![feature(fmt_as_str)]
#![no_std]

extern crate libc;
use libc::{c_char, printf};

fn space() -> &'static str {
    "\n\0"
}

pub fn print_main(arg: &str) {
    unsafe { printf(space().as_ptr() as *const c_char); }
    unsafe { printf(arg.as_ptr() as *const c_char); }
}
