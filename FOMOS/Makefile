all: main.c setup.c commands/help/help.c commands/restart/restart.c commands/shutdown/shutdown.c commands/Fopen/Fopen.c commands/help/rndmstf.c commands/pwd/pwd.c commands/ls/ls.rs commands/mkdir/mkdir.rs commands/sudo/sudo.rs compileSudoShutdown.sh
		gcc -Wall main.c -o main
		gcc -Wall setup.c -o setup
		gcc -Wall commands/help/help.c -o commands/help/help
		gcc -Wall commands/restart/restart.c -o commands/restart/restart
		gcc -Wall commands/shutdown/shutdown.c -o commands/shutdown/shutdown
		rustc commands/Fopen/Fopen.rs
		gcc -Wall commands/help/rndmstf.c -o commands/help/rndmstf
		gcc -Wall commands/pwd/pwd.c -o commands/pwd/pwd
		rustc commands/ls/ls.rs
		rustc commands/mkdir/mkdir.rs
		rustc commands/sudo/sudo.rs
		sh compileSudoShutdown.sh

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
		@rm -rf commands/pwd/pwd
		@rm -rf commands/ls/ls
		@rm -rf commands/mkdir/mkdir
		@rm -rf commands/mkdir/mkdir
		@rm -rf commands/sudo/sudo
