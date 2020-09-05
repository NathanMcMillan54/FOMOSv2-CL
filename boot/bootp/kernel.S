	.globl	kernel_start
kernel_start:
	.incbin	"boot/zImage"
	.globl	kernel_end
kernel_end:
	.align	2
