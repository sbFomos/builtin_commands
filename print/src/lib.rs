#![no_std]

extern crate libc;
use libc::printf;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
type c_char = u8;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_char = i8;


pub extern fn print_main(arg: &str) {
    unsafe { printf(arg.as_ptr() as *const c_char); }
}
