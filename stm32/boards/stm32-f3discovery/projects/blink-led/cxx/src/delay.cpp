#include "delay.h"
#include "common.h"

void init_delay() {
    // NOTE: APB1CLK is always enabled.

    // Divide APB1CLK by 16 (RCC_CFGR.PPRE1 = 111).
    // SYSCLK is driven by HSI, which runs at 8 MHz.
    // The desired TIM6CLK is 1 MHz, but if APB1CLK is divided at all, it's multiplied by 2 prior to setting TIM6CLK.
    // Therefore, divide APB1CLK by 16 to get a TIM6CLK of 1 MHz.
    *REG(RCC_CFGR) |= (0b111 << PPRE1);
    
    // Enable TIM6CLK.
    *REG(RCC_APB1ENR) |= (1 << TIM6EN);

    // Set the auto-reload register to 1,000.
    // TIM6CLK runs at 1 MHz, which has a period of 1 microsecond. 1 millisecond has passed when the counter reaches 1,000.
    *REG(TIM6_ARR) = 1000;

    // The UIF status bit is copied to TIM6_CNT(31).
    *REG(TIM6_CR1) |= (1 << UIFREMAP);
}

void delay_ms(int num_ms) {
    // Reset the counter to 0.
    *REG(TIM6_CNT) = 0;

    // Enable the counter.
    *REG(TIM6_CR1) |= (1 << CEN);

    for (; num_ms > 0; --num_ms) {
        // Wait for the update interrupt flag to be set.
        while ((*REG(TIM6_CNT) & (1 << UIFCPY)) == 0);

        // Clear the update interrupt flag.
        *REG(TIM6_SR) &= ~(1 << UIF);
    }

    // Disable the counter.
    *REG(TIM6_CR1) &= ~(1 << CEN);
}