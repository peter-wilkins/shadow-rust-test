all: driver target/debug/libshadow_plugin_rust_test.so

target/debug/libshadow_plugin_rust_test.so:
	cargo build

driver: driver.o
	$(CC) -o $@ $< -ldl

driver.o: src/driver.c
	$(CC) -c $< -o $@

install: target/debug/libshadow_plugin_rust_test.so
	cp $< ~/.shadow/plugins/

clean:
	rm -rf driver driver.o
	rm -rf ~/.shadow/plugins/libshadow_plugin_rust_test.so
	cargo clean

.PHONY: all clean install
