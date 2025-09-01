    .section .Reset, "ax", %progbits
    .align 1
    .syntax unified

    .global Reset
    .type Reset, %function
    .thumb_func
Reset:
    @ Initialize SRAM.
    ldr r1, =__sidata
    ldr r2, =__sdata
    ldr r3, =__edata
0:
    ldm r1!, {r0}
    stm r2!, {r0}
    cmp r2, r3
    bcc 0b

    @ Branch to main, which never returns.
    b main


    .global DefaultHandler
    .type DefaultHandler, %function
    .thumb_func
DefaultHandler:
    b DefaultHandler
