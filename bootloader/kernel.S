# This was copied from https://github.com/torvalds/linux/tree/master/arch/arm/boot/bootp

	.globl	kernel_start
kernel_start:
    # I think zImage is supposed to be a directory that is made by a makefile in linux
    # zImage is probably supposed to go in this directory
	.incbin	"zImage"
	.globl	kernel_end
kernel_end:
	.align	2
