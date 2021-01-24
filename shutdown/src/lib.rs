#![no_std]

extern crate libc;
extern crate asku;

extern "C" {
    fn shutdown(s: i32) -> !;
}

fn start_shutdown(ss: &str) {
    let seconds = unsafe { libc::atoi(ss.as_ptr() as *const libc::c_char) };
    unsafe { libc::printf("Shutting down...\n".as_ptr() as *const libc::c_char); }
    unsafe { shutdown(seconds) }
}

pub fn shutdown_main(arg: &str) -> i32 {
    if asku::asku_main() == true {
        start_shutdown(arg);
    } return 0;
}
