    @ Registers
    .set .LRCC_AHBENR, 0x40021014
    .set .LGPIOE_MODER, 0x48001000
    .set .LGPIOE_BSRR, 0x48001018

    @ Bit fields
    .set .LIOPEEN, 21
    .set .LMODER8, 16
    .set .LBS8, 8
    .set .LBR8, 24

    .section .text.led, "ax", %progbits
    .align 1
    .syntax unified

    /*
        Performs the initialization required to use the LED.
    */
    .global init_led
    .type init_led, %function
    .thumb_func
init_led:
    @ Enable AHB clock for Port E pins.
    @ RCC_AHBENR |= 1 << IOPEEN
    ldr r0, =.LRCC_AHBENR
    ldr r1, [r0]
    movs r2, 1
    lsl r2, .LIOPEEN
    orrs r1, r2
    str r1, [r0]

    @ Configure Port E Pin 8 as a push-pull output.
    @ GPIOE_MODER &= ~(0b11 << MODER8)
    @ GPIOE_MODER |= 1 << MODER8
    ldr r0, =.LGPIOE_MODER
    ldr r1, [r0]
    movs r2, 3
    lsl r2, .LMODER8
    mvn r3, r2
    ands r1, r3
    movs r2, 1
    lsl r2, .LMODER8
    orrs r1, r2
    str r1, [r0]
    bx lr
    .size init_led, . - init_led

    /*
        Set the state of the LED.

        @param state (r0)   The desired state of the LED.
                                0: LED off
                                1: LED on
    */
    .global set_led
    .type set_led, %function
    .thumb_func
set_led:
    ldr r1, =.LGPIOE_BSRR
    ldr r2, [r1]
    movs r3, 1
    cmp r0, 1
    bne 0f
    @ Turn the LED on.
    lsl r3, .LBS8
    orrs r2, r3
    str r2, [r1]
    b 1f
0:
    @ Turn the LED off.
    lsl r3, .LBR8
    orrs r2, r3
    str r2, [r1]
1:
    bx lr
    .size set_led, . - set_led
