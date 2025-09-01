#include "led.h"
#include "common.h"

void init_led() {
    // Enable AHB clock for Port E pins.
    *REG(RCC_AHBENR) |= (1 << IOPEEN);

    // Configure Port E Pin 8 as a push-pull output.
    *REG(GPIOE_MODER) |= (1 << MODER8);
}

void set_led(LedState state) {
    switch (state) {
        case LedState::ON:
            *REG(GPIOE_BSRR) |= (1 << BS8);
            break;
        case LedState::OFF:
            *REG(GPIOE_BSRR) |= (1 << BR8);
            break;
        default:
            return;
    }
}

LedState toggle_led_state(LedState state) {
    switch (state) {
        case LedState::ON:
            return LedState::OFF;
        case LedState::OFF:
            return LedState::ON;
        default:
            return LedState::OFF;
    };
}