    .section .data.main, "aw"
    .align 2
    .type led_state, %object
    .size led_state, 1
led_state:
    .byte 1

    .section .text.main, "ax", %progbits
    .align 1
    .syntax unified

    .global main
    .type main, %function
    .thumb_func
main:
    bl init_led
    bl init_delay

0:
    ldr r12, =led_state
    ldrb r0, [r12]
    bl set_led
    
    movw r0, 1000
    bl delay_ms

    @ Toggle the LED state.
    ldr r12, =led_state
    ldrb r0, [r12]
    rsb.w r0, r0, 1
    strb r0, [r12]

    b 0b
    .size main, . - main
