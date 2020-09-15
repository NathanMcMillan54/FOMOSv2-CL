all: sleep.s drivers/keyboard/keyboard.cpp
		gcc -o sleep sleep.s
		g++ -static drivers/keyboard/keyboard.cpp -o drivers/keyboard/keyboard

clean:
		@rm -rf sleep
		@rm -rf drivers/keyboard/keyboard

# TODO figure out how to compile arm assembly
