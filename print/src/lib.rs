#![no_std]
#![feature(lang_items)]

mod lang;

const L1: &str = "Never gonna give you up\n\0";
const L2: &str = "Never gonna let you down\n\0";
const L3: &str = "Never gonna run around and desert you\n\0";
const L4: &str = "Never gonna make you cry\n\0";
const L5: &str = "Never gonna say goodbye\n\0";
const L6: &str = "Never gonna tell a lie and hurt you\n\0";
const RICK: &str = "\n__rick_astley__\n\0";

unsafe fn easter_egg() {
    printf(L1.as_ptr() as *const _);
    printf(L2.as_ptr() as *const _);
    printf(L3.as_ptr() as *const _);
    printf(L4.as_ptr() as *const _);
    printf(L5.as_ptr() as *const _);
    printf(L6.as_ptr() as *const _);
    printf(RICK.as_ptr() as *const _);
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
type c_char = u8;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_char = i8;

extern "C" { fn printf(format: *const c_char) -> i32; }

#[no_mangle]
pub extern "C" fn print(arg: &str) {
    if arg == "__rick_astley__" {
        unsafe { easter_egg(); }
    } else {
        unsafe { printf(arg.as_ptr() as *const _); }
        unsafe { printf("\n\0".as_ptr() as *const _); }
    }
}
