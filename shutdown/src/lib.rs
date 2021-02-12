#![no_std]

extern crate fk_std;
use fk_std::libc::{atoi, printf};
extern crate asku;

extern "C" {
    fn shutdown(s: i32) -> !;
}

fn start_shutdown(ss: &str) {
    let seconds = unsafe { atoi(ss.as_ptr() as *const fk_std::c_char) };
    unsafe { printf("Shutting down...\n".as_ptr() as *const fk_std::c_char); }
    unsafe { shutdown(seconds) }
}

pub fn shutdown_main(arg: &str) -> i32 {
    if asku::asku_main() == true {
        start_shutdown(arg);
    } return 0;
}
