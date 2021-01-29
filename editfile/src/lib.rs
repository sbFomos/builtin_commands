#![no_std]


extern "C" {
    fn editfile(file: &str);
}

pub fn editfile_main(arg: &str) {
    unsafe { editfile(arg); }
}
