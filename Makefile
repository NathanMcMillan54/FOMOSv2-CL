# obj-m := linux_FOMOSv2.o
linux_FOMOSv2 := FOMOSv2CL

ifeq ($(shell uname),linux-gnu)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: FOMOSv2CL

target:
	mkdir -p $@

FOMOSv2CL: linux_FOMOSv2.o target/debug/libFOMOS.so
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libFOMOS.so: src/lib.rs Cargo.toml
	cargo build

target/linux_FOMOSv2.o: linux_FOMOSv2.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target/
	rm -rf *.a
	rm -rf *.o
