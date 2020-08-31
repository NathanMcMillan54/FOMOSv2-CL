all: configure
		arm-none-eabi-gcc -c -mcpu=arm926ej-s -g main.c -o boot/main.o
		arm-none-eabi-as -mcpu=arm926ej-s -g startup.s -o boot/startup.o
		arm-none-eabi-id -T start.ld boot/start.o boot/startup.o -o boot/main.elf

clean:
	@rm -rf boot/main.o
	@rm -rf boot/startup.o
	@rm -rf boot/main.elf

disas:
	arm-none-eabi-objdump boot/main.elf
