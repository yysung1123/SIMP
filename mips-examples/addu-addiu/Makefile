addu-addiu.bin: addu-addiu.s
	mipsel-linux-gnu-gcc -mips32 -Wl,-Ttext=0x0 -nostdlib -o addu-addiu addu-addiu.s
	mipsel-linux-gnu-objcopy -S -O binary -j .text addu-addiu addu-addiu.bin

clean:
	rm -f addu-addiu
	rm -f addu-addiu.bin