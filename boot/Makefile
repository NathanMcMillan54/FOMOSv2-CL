.POSIX:

MAIN := main.img

.PHONY: clean run

$(MAIN): main.s
	as -o main.o '$<'
	ld --oformat binary -o '$@' -Ttext 0x7C00 main.o

clean:
	rm -f '$(MAIN)'
	rm -f main.o
