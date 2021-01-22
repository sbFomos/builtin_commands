extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/password.c");
    cc::Build::new()
        .file("src/password.c")
        .compile("asku_password");
}