# This was copied from https://github.com/torvalds/linux/tree/master/arch/arm/boot/bootp

	.globl	kernel_start
kernel_start:
	.incbin	"../FOMOS/zImage"
	.globl	kernel_end
kernel_end:
	.align	2
