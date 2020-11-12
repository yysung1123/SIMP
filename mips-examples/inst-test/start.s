.extern main

.equ STACKTOP, 0x80000000 + 0x8000000

.text
.global __start
__start:
    li  $sp, STACKTOP
    j   main
