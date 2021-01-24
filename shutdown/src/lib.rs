#![no_std]

extern crate libc;

extern "C" {
    fn shutdown(s: i32) -> !;
}


pub fn shutdown_main(arg: &str) -> ! {
    let seconds = unsafe { libc::atoi(arg.as_ptr() as *const libc::c_char) };
    unsafe { libc::printf("Shutting down...\n".as_ptr() as *const libc::c_char); }
    unsafe { shutdown(seconds) }
}
