#pragma once

#include <cstdint>

static constexpr uint32_t RCC_AHBENR __attribute__ ((section (".rodata.led"))) = 0x40021014;
static constexpr uint32_t IOPEEN __attribute__ ((section (".rodata.led"))) = 21;
static constexpr uint32_t GPIOE_MODER __attribute__ ((section (".rodata.led"))) = 0x48001000;
static constexpr uint32_t MODER8 __attribute__ ((section (".rodata.led"))) = 16;
static constexpr uint32_t GPIOE_BSRR __attribute__ ((section (".rodata.led"))) = 0x48001018;
static constexpr uint32_t BS8 __attribute__ ((section (".rodata.led"))) = 8;
static constexpr uint32_t BR8 __attribute__ ((section (".rodata.led"))) = 24;

enum class LedState : uint8_t {
    ON, OFF
};

void init_led() __attribute__ ((section (".text.led")));
void set_led(LedState state) __attribute__ ((section (".text.led")));
LedState toggle_led_state(LedState state) __attribute__ ((section (".text.led")));