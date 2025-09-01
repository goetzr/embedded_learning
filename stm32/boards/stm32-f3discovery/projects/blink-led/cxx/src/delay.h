#pragma once

#include <cstdint>

static constexpr uint32_t RCC_BASE __attribute__ ((section (".rodata.delay"))) = 0x40021000;
static constexpr uint32_t RCC_CFGR __attribute__ ((section (".rodata.delay"))) = RCC_BASE + 0x04;
static constexpr uint32_t PPRE1 __attribute__ ((section (".rodata.delay"))) = 8;
static constexpr uint32_t RCC_APB1ENR __attribute__ ((section (".rodata.delay"))) = RCC_BASE + 0x1C;
static constexpr uint32_t TIM6EN __attribute__ ((section (".rodata.delay"))) = 4;

static constexpr uint32_t TIM6_BASE __attribute__ ((section (".rodata.delay"))) = 0x40001000;
static constexpr uint32_t TIM6_CR1 __attribute__ ((section (".rodata.delay"))) = TIM6_BASE + 0x00;
static constexpr uint32_t UIFREMAP __attribute__ ((section (".rodata.delay"))) = 11;
static constexpr uint32_t CEN __attribute__ ((section (".rodata.delay"))) = 0;
static constexpr uint32_t TIM6_SR __attribute__ ((section (".rodata.delay"))) = TIM6_BASE + 0x10;
static constexpr uint32_t UIF __attribute__ ((section (".rodata.delay"))) = 0;
static constexpr uint32_t TIM6_CNT __attribute__ ((section (".rodata.delay"))) = TIM6_BASE + 0x24;
static constexpr uint32_t UIFCPY __attribute__ ((section (".rodata.delay"))) = 31;
static constexpr uint32_t TIM6_ARR __attribute__ ((section (".rodata.delay"))) = TIM6_BASE + 0x2C;

void init_delay() __attribute__ ((section (".text.delay")));
void delay_ms(int num_ms) __attribute__ ((section (".text.delay")));