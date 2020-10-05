all: keymap_rust.rs keymap_c.c
	gcc -c -o keymap_c.o keymap_c.c
	rustc --emit=obj --codegen panic=abort -o keymap_rust.o keymap_rust.rs
	gcc -o keyboard_drivers keymap_c.o keymap_rust.o

clean:
	@rm -rf *.o
	@rm -rf keyboard_drivers

run:
	@./keyboard_drivers
