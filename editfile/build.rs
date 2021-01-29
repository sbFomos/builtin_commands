extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/editfile.c");
    cc::Build::new()
        .file("src/editfile.c")
        .compile("editfile");
}
