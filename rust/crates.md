# Crates Overview

## Crates specific to the STM32 F3 discover board

- stm32f3 (device support crate)
- stm32f3-discovery (board support crate)
- stm32f3xx-hal (peripheral access API for STM32F3)

## ARM Cortex-M crates

- cortex-m
- cortex-m-rt

## Generic crates

- embedded-hal (HAL for embedded systems)
- embedded-dma (traits to aid in creation of sound DMA abstractions)
- accelerometer
- lsm303dlhc (accelerometer + compass)
- switch-hal (HAL for input and output switches
  - (buttons, switches, leds, transistors))
- bare-metal (abstractions common to bare metal systems)
- micromath (fast embedded floating point math)
- bitfield (provides macros to generate bitfield-like struct)
- volatile-register (volative access to memory-mapped hardware registers)
- r0 (initialization code ("crt0") written in Rust)
- generic-array (generic array types)

## Other

- [stm32-rs](https://github.com/stm32-rs/stm32-rs)
