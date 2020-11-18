all:
	mkdir FOMOSv2CL_boot
	as --64 -o init.o init.S
	ld -o FOMOSv2CL_boot/init init.o
	find . | cpio -o -H newc | gzip > FOMOSv2CL_boot/rootfs.cpio.gz

clean:
	rm -rf *.o
	rm -rf FOMOSv2CL_boot/