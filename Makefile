all: CPU.cpp
		g++ CPU.cpp -o CPU
		gcc -static FOMOS.c -o FOMOS

run:
		@./FOMSO

clean:
		@rm -rf CPU

# don't add FOMOS, it'll delete the directory
