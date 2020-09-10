all: CPU.cpp
		g++ CPU.cpp -o CPU
		gcc -static FOMOS.c -o FOMOS

run:
		@./CPU

clean:
		@rm -rf CPU
