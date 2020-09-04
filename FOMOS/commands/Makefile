all: installFMOE.sh restart/restart.c shutdown/shutdown.c help/help.c help/rndmstf.c
		gcc -Wall restart/restart.c -o restart/restart
		gcc -Wall shutdown/shutdown.c -o shutdown/shutdown
		gcc -Wall help/help.c -o help/help
		gcc -Wall help/rndmstf.c -o help/rndmstf
		sh installFMOE.sh

clean:
		@rm -rf restart/restart
		@rm -rf shutdown/shutdown
		@rm -rf help/help
		@rm -rf FMOE/
