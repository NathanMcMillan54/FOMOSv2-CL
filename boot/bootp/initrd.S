# This was copied from https://github.com/torvalds/linux/tree/master/arch/arm/boot/bootp

	.type	initrd_start,
	.globl	initrd_start
initrd_start:
	.incbin	INITRD
	.globl	initrd_end
initrd_end: