print: lib exe

lib:
	cargo build --target=arm-unknown-linux-gnueabihf
	cargo build --target=armv7-unknown-linux-gnueabihf
	cargo build --target=x86_64-unknown-linux-gnu

exe:
	arm-linux-gnueabihf-gcc -static src/main.c target/arm-unknown-linux-gnueabihf/debug/libprint.a -o armv6_print
	arm-linux-gnueabihf-gcc -static src/main.c target/armv7-unknown-linux-gnueabihf/debug/libprint.a -o armv7_print
	x86_64-linux-gnu-gcc -static src/main.c target/x86_64-unknown-linux-gnu/debug/libprint.a -o x86_print

clean:
	rm -rf arm6_print
	rm -rf arm7_print
	rm -rf x86_print
