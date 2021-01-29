#![no_std]

extern crate editfile;

extern "C" {
    fn makefile(file_name: &str);
}

pub fn makefile_main(arg: &str) {
    unsafe { makefile(arg); }
    editfile::editfile_main(arg);
}
