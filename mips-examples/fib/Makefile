fib.bin: fib.o
	mipsel-linux-gnu-objcopy -S -O binary -j .text fib.o fib.bin

fib.o: start.o fib.s
	mipsel-linux-gnu-gcc -mips32 -Wl,-Ttext=0xbfc00000 -nostdlib -o fib.o start.o fib.s

fib.s: fib.c
	mipsel-linux-gnu-gcc -mips32 -S -o fib.s fib.c

start.o: start.s
	mipsel-linux-gnu-gcc -mips32 -mabicalls -c -o start.o start.s

clean:
	rm -f start.o
	rm -f fib.s
	rm -f fib.o
	rm -f fib.bin
