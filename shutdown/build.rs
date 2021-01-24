extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/shutdown.c");
    cc::Build::new()
        .file("src/shutdown.c")
        .compile("shutdown");
}
