all: lib

lib:
	cargo build --target=arm-unknown-linux-gnueabihf
	cargo build --target=armv7-unknown-linux-gnueabihf
	cargo build --target=x86_64-unknown-linux-gnu
	mv target/arm-unknown-linux-gnueabihf/debug/libbuiltin_commands.rlib armv6_builtin_commands
	mv target/armv7-unknown-linux-gnueabihf/debug/libbuiltin_commands.rlib armv7_builtin_commands
	mv target/x86_64-unknown-linux-gnu/debug/libbuiltin_commands.rlib x86_builtin_commands

clean:
	rm -rf target/
	rm -rf armv6_builtin_commands
	rm -rf armv7_builtin_commands
	rm -rf x86_builtin_commands
