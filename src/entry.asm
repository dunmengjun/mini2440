@ .equ WTCON, 0x53000000
@ .equ INTMASK, 0x4a000008
@ .equ INTSUBMARK, 0x4a00001c
.section .text.entry
.code 32
.extern rust_main
.globl _start
.align 4
_start: 
    @ msr cpsr, #0xc0 | 0x13
    @ ldr r0, =WTCON
    @ mov r1, #0
    @ str r1, [r0]

    @ ldr r0, =INTMASK
    @ ldr r1, =0xffffffff
    @ str r1, [r0]

    @ ldr r0, =INTSUBMARK
    @ ldr r1, =0x7fff
    @ str r1, [r0]

    ldr sp, =0x34000000
    bl rust_main
