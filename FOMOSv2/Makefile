obj-y := linux_FOMOSv2.o

ifeq ($(shell uname),linux-gnu)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/fomosv2cl

target:
	mkdir -p $@

target/fomosv2cl: target/run_fomosv2.o target/debug/libFOMOS.so
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libFOMOS.so: src/lib.rs Cargo.toml
	cargo build

target/run_fomosv2.o: run_fomosv2.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target/
	rm -rf *.a
	rm -rf *.o
