print: lib exe

lib:
	cargo build

exe:
	gcc -static src/main.c target/debug/libprint.a -o print
