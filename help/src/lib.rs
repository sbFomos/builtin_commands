#![no_std]

extern crate libc;

mod asku;
mod help;
mod print;
mod shutdown;

unsafe fn all() {
    libc::printf(asku::ASKU.as_ptr() as *const libc::c_char);
    libc::printf(print::PRINT.as_ptr() as *const libc::c_char);
    libc::printf(shutdown::SHUTDOWN.as_ptr() as *const libc::c_char);
    libc::printf(help::HELP.as_ptr() as *const libc::c_char);
}

pub fn help_main(arg: &str) {
    if arg == "all" {
        unsafe { all(); }
    } else if arg == "asku" {
        unsafe { libc::printf(asku::ASKU.as_ptr() as *const libc::c_char); }
    } else if arg == "print" {
        unsafe { libc::printf(print::PRINT.as_ptr() as *const libc::c_char); }
    } else if arg == "help" {
        unsafe { libc::printf(help::HELP.as_ptr() as *const libc::c_char); }
    } else if arg == "shutdown" {
        unsafe { libc::printf(shutdown::SHUTDOWN.as_ptr() as *const libc::c_char); }
    }
}
