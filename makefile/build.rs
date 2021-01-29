extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/makefile.c");
    cc::Build::new()
        .file("src/makefile.c")
        .compile("makefile");
}
