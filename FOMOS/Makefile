all: main.c setup.c makePower.sh
		gcc -Wall main.c -o main
		gcc -Wall setup.c -o setup
		sh makePower.sh

run:
		@./main

# for now it'll just run main

clean:
		@rm -rf main
		@rm -rf setup

