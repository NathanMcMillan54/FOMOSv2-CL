all: main.c setup.c commands/help/help.c commands/restart/restart.c commands/shutdown/shutdown.c commands/Fopen/Fopen.c
		gcc -Wall main.c -o main
		gcc -Wall setup.c -o setup
		gcc -Wall commands/help/help.c -o commands/help/help
		gcc -Wall commands/restart/restart.c -o commands/restart/restart
		gcc -Wall commands/shutdown/shutdown.c -o commands/shutdown/shutdown
		gcc -Wall commands/Fopen/Fopen.c -o commands/Fopen/Fopen
		gcc -Wall commands/help/rndmstf.c -o commands/help/rndmstf

run:
		@./main

# for now it'll just run main

clean:
		@rm -rf main
		@rm -rf setup
		@rm -rf commands/help/help
		@rm -rf commands/restart/restart
		@rm -rf commands/shutdown/shutdown
		@rm -rf commands/FMOE/
		@rm -rf commands/Fopen/Fopen
