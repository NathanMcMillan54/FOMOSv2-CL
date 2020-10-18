PWD := $(shell pwd)
obj-m += start_modules.o

all:
	make ARCH=arm CROSS_COMPILE=$(CROSS) -C $(KERNEL) SUBDIRS=$(PWD) modules
clean:
	make -C $(KERNEL) SUBDIRS=$(PWD) clean
