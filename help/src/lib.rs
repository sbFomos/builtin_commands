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
    unsafe {
        if arg == "all" {
            all();
        } else if arg == "asku" {
            libc::printf(asku::ASKU.as_ptr() as *const libc::c_char);
        } else if arg == "help" {
            libc::printf(help::HELP.as_ptr() as *const libc::c_char);
        } else if arg == "print" {
            libc::printf(print::PRINT.as_ptr() as *const libc::c_char);
        } else if arg == "shutdown" {
            libc::printf(shutdown::SHUTDOWN.as_ptr() as *const libc::c_char);
        }
    }
}
