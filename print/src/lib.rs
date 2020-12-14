#![no_std]

extern crate fk_std;
use fk_std::{printf};

const L1: &str = "Never gonna give you up\n\0";
const L2: &str = "Never gonna let you down\n\0";
const L3: &str = "Never gonna run around and desert you\n\0";
const L4: &str = "Never gonna make you cry\n\0";
const L5: &str = "Never gonna say goodbye\n\0";
const L6: &str = "Never gonna tell a lie and hurt you\n\0";
const RICK: &str = "__rick_astley__\n\0";

unsafe fn easter_egg() {
    printf(L1.as_ptr() as *const _);
    printf(L2.as_ptr() as *const _);
    printf(L3.as_ptr() as *const _);
    printf(L4.as_ptr() as *const _);
    printf(L5.as_ptr() as *const _);
    printf(L6.as_ptr() as *const _);
    printf(RICK.as_ptr() as *const _);
}

pub fn main(arg: &str) {
    if arg == "__rick_astley__" {
        unsafe { easter_egg(); }
    } else {
        unsafe { printf(arg.as_ptr() as *const _); }
    }
}
