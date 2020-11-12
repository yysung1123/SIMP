inst-test.bin: inst-test.o
	mipsel-linux-gnu-objcopy -S -O binary -j .text inst-test.o inst-test.bin

inst-test.o: start.o inst-test.s
	mipsel-linux-gnu-gcc -mips32 -Wl,-Ttext=0xbfc00000 -nostdlib -o inst-test.o start.o inst-test.s

inst-test.s: inst-test.c
	mipsel-linux-gnu-gcc -mips32 -S -o inst-test.s inst-test.c

start.o: start.s
	mipsel-linux-gnu-gcc -mips32 -c -o start.o start.s

clean:
	rm -f start.o
	rm -f inst-test.s
	rm -f inst-test.o
	rm -f inst-test.bin
