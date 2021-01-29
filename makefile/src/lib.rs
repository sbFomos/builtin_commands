#![no_std]

extern "C" {
    fn makefile(file_name: &str);
}

pub fn makefile_main(arg: &str) {
    unsafe { makefile(arg); }
}
