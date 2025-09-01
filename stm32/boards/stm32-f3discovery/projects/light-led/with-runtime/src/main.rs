#![no_main]
#![no_std]

// Some panic handler needs to be included.
use panic_halt as _;

use core::ptr;
use cortex_m_rt::entry;

const AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
const MODER: *mut u32 = 0x4800_1000 as *mut u32;
const BSRR: *mut u32 = 0x4800_1018 as *mut u32;

const IOPEEN: u32 = 21;
const MODER8: u32 = 16;
const BS8: u32 = 8;

#[entry]
unsafe fn main() -> ! {
    // Light the blue LED, which is connected to PE8.

    // Must enable the port E clock to enable the use of its pins.
    // Page 148 of STM32 F3 Reference Manuals:
    //  Note: When the peripheral clock is not active, the peripheral register values may not be readable by software
    //        and the returned value is always 0x0.
    let mut val = ptr::read_volatile(AHBENR);
    val |= 1 << IOPEEN;
    ptr::write_volatile(AHBENR, val);

    // Configure pin 8 as a push-pull output.
    // MODER8 = 01 (output)
    // PUPDR8 = 00 (push-pull, which is the reset state, so no need to set it)
    val = ptr::read_volatile(MODER);
    val = (val & !(0b11 << MODER8)) | (0b01 << MODER8);
    ptr::write_volatile(MODER, val);

    // Set pin.
    // BSRR.BS8 = 1
    ptr::write_volatile(BSRR, 1u32 << BS8);

    loop {}
}