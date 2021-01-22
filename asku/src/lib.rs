#![no_std]

#[macro_use]
extern crate fk_std;

extern "C" {
    fn checkPassword() -> i32;
}

pub fn asku_main() {
    for mut i in 0..3 {
        printfk!("[ asku ] Enter user password: \0");
        let password = unsafe { checkPassword() };
        if password == 0 {
            printfk!("asku - Incorrect Password\n\0");
        } else {
            i = 3
        }
    }
}