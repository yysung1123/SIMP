# SIMP: SImple Mips Processor emulator

This is a MIPSEL32 processor emulator in Rust.

Inspired from https://github.com/d0iasm/rvemu-for-book

![simp](https://img.yysung.tw/img/c85e723792cc199a5aebaee154b09288a4b51891c438504b236508999221ed7d.jpg)

## Build and Run

addu-addiu
```
$ make -C mips-examples/addu-addiu
$ cargo run mips-examples/addu-addiu/addu-addiu.bin
```

fibonacci
```
$ make -C mips-examples/fib
$ cargo run mips-examples/fib/fib.bin
```
