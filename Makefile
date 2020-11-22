all: FOMOSv2
	cp -r target/debug/libFOMOSv2_CL.a libFOMOSv2_CL.a
	gcc -static linux/main.c libFOMOSv2_CL.a -o init
	find . | cpio -o -H newc | gzip > rootfs.cpio.gz

FOMOSv2:
	cargo build

clean:
	rm -rf target/
	rm -rf init
	rm -rf *.a
	rm -rf *.gz
	rm -rf *.o
