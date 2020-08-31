BIN_DIR=bin

all: configure
		arm-none-eabi-gcc -c -mcpu=arm926ej-s -g main.c -o $(BIN_DIR)/main.o
		arm-none-eabi-as -mcpu=arm926ej-s -g startup.s -o $(BIN_DIR)/startup.o
		arm-none-eabi-id -T start.ld $(BIN_DIR)/start.o $(BIN_DIR)/startup.o -o $(BIN_DIR)/main.elf

configure:
	@mkdir -p $(BIN_DIR)

clean:
	@rm -rf $(BIN_DIR)

disas:
	arm-none-eabi-objdump $(BIN_DIR)/main.elf
