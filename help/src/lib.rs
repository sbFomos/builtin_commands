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
    unsafe { libc::printf(arg.as_ptr() as *const libc::c_char); }
    let mut i = 0;
    if arg == "all" {
        i = i + 0;
    } else if arg == "asku" {
        i = i + 2;
    } else if arg == "print" {
        i = i + 1;
    } else if arg == "help" {
        i = i + 4;
    } else if arg == "shutdown" {
        i = i + 3;
    }
    if i == 0 {
        unsafe { all(); }
    } else if i == 1 {
        unsafe { libc::printf(print::PRINT.as_ptr() as *const libc::c_char); }
    } else if i == 2 {
        unsafe { libc::printf(asku::ASKU.as_ptr() as *const libc::c_char); }
    } else if i == 3 {
        unsafe { libc::printf(shutdown::SHUTDOWN.as_ptr() as *const libc::c_char); }
    } else if i == 4 {
        unsafe { libc::printf(help::HELP.as_ptr() as *const libc::c_char); }
    }
}
