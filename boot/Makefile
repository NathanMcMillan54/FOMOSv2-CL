all: app.c bootloader.c
		gcc -static app.c -o app.elf
		gcc -Wall bootloader.c -o bootloader

clean:
		@rm -rf app.elf
		@rm -rf bootloader

run:
		@./app.elf
