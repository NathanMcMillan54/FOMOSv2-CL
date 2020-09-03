all: CPU.cpp
		g++ CPU.cpp -o CPU

run:
		@./CPU

clean:
		@rm -rf CPU
