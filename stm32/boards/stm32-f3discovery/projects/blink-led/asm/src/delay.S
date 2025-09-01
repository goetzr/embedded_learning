    @ RCC
    .set .LRCC_BASE, 0x40021000
    .set .LRCC_CFGR, .LRCC_BASE + 0x04
    .set .LPPRE1, 8
    .set .LRCC_APB1ENR, .LRCC_BASE + 0x1C
    .set .LTIM6EN, 4

    @ TIM6
    .set .LTIM6_BASE, 0x40001000
    .set .LTIM6_CR1, .LTIM6_BASE + 0x00
    .set .LUIFREMAP, 11
    .set .LCEN, 0
    .set .LTIM6_SR, .LTIM6_BASE + 0x10
    .set .LUIF, 0
    .set .LTIM6_CNT, .LTIM6_BASE + 0x24
    .set .LUIFCPY, 31
    .set .LTIM6_ARR, .LTIM6_BASE + 0x2C

    .section .text.delay, "ax", %progbits
    .align 1
    .syntax unified

    /*
        Performs the initialization required to use the delay_ms function.
    */
    .global init_delay
    .type init_delay, %function
    .thumb_func
init_delay:
    @ NOTE: APB1CLK is always enabled.

    @ Divide APB1CLK by 16 (RCC_CFGR.PPRE1 = 111).
    @ SYSCLK is driven by HSI, which runs at 8 MHz.
    @ The desired TIM6CLK is 1 MHz, but if APB1CLK is divided at all, it's multiplied by 2 prior to setting TIM6CLK.
    @ Therefore, divide APB1CLK by 16 to get a TIM6CLK of 1 MHz.
    ldr r10, =.LRCC_CFGR
    ldr r0, [r10]
    orrs r0, 0b111 << .LPPRE1
    str r0, [r10]

    @ Enable TIM6CLK.
    ldr r10, =.LRCC_APB1ENR
    ldr r0, [r10]
    orrs r0, 1 << .LTIM6EN
    str r0, [r10]

    @ Set the auto-reload register to 1,000.
    @ TIM6CLK runs at 1 MHz, which has a period of 1 microsecond. 1 millisecond has passed when the counter reaches 1,000.
    ldr r10, =.LTIM6_ARR
    movw r0, 1000
    str r0, [r10]

    @ The UIF status bit is copied to TIM6_CNT(31).
    ldr r10, =.LTIM6_CR1
    ldr r0, [r10]
    orrs r0, 1 << .LUIFREMAP
    str r0, [r10]
    bx lr
    .size init_delay, . - init_delay


    /*
        Delays execution for the specified number of milliseconds.

        @param num_ms (r0)  The number of milliseconds to delay execution.
    */
    .global delay_ms
    .type delay_ms, %function
    .thumb_func
delay_ms:
    ldr r10, =.LTIM6_CNT
    ldr r11, =.LTIM6_CR1
    ldr r12, =.LTIM6_SR

    @ Reset the counter to 0.
    movs r1, 0
    str r1, [r10]

    @ Enable the counter.
    ldr r1, [r11]
    orrs r1, 1 << .LCEN
    str r1, [r11]

    @ Wait for the update interrupt flag to be set.
0:  ldr r1, [r10]
    tst r1, 1 << .LUIFCPY
    beq 0b

    @ Clear the update interrupt flag.
    ldr r2, [r12]
    bic r2, 1 << .LUIF
    str r2, [r12]

    subs r0, 1
    bne 0b

    @ Disable the counter.
    ldr r1, [r11]
    bic r1, 1 << .LCEN
    str r1, [r11]
    bx lr
    .size delay_ms, . - delay_ms
