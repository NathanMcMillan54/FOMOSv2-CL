all:
	gcc -static main.c -o init
	cp -r init initramfs/init
	cd initramfs/
	find . | cpio -H newc -o > initramfs.cpio

clean:
	rm -rf *.cpio
	rm -rf init
	rm -rf initramfs/init
