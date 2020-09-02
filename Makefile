all: CPU.cpp
		g++ CPU.cpp -o CPU

clean:
		@rm -rf CPU

run:
		@./CPU