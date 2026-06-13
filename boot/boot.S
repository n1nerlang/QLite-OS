.section .text
.global _start
.extern kernel_main

_start:
    cli
    call kernel_main
1:
    hlt
    jmp 1b
