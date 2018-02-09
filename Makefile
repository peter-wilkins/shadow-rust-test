all: driver target/debug/libshadow_rust_test.so

target/debug/libshadow_rust_test.so:
	cargo build

driver: driver.o
	$(CC) -o $@ $< -ldl

driver.o: src/driver.c
	$(CC) -c $< -o $@

clean:
	rm -rf driver driver.o
	cargo clean

.PHONY: all clean
