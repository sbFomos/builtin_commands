#![no_std]

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
type c_char = u8;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_char = i8;

extern "C" { fn printf(format: *const c_char) -> i32; }

pub extern fn print_main(arg: &str) {
    unsafe { printf(arg.as_ptr() as *const c_char); }
    unsafe { printf("\n".as_ptr() as *const c_char); }
}
