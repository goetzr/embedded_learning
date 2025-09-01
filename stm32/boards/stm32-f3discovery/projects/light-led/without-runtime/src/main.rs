#![no_std]
#![no_main]
#![allow(non_snake_case)]

#[export_name = "Reset"]
#[link_section = ".text"]
pub unsafe extern "C" fn main() -> ! {
    use core::ptr;
    // Light the blue LED, which is connected to PE8.

    // Must enable the port E clock to enable the use of its pins.
    // Page 148 of STM32 F3 Reference Manuals:
    //  Note: When the peripheral clock is not active, the peripheral register values may not be readable by software
    //        and the returned value is always 0x0.
    const AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
    let mut ahbenr_val = ptr::read_volatile(AHBENR);
    ahbenr_val |= 1 << 21;
    ptr::write_volatile(AHBENR, ahbenr_val);

    // Configure pin 8 as a push-pull output.
    // MODER8 = 01 (output)
    // PUPDR8 = 00 (push-pull, which is the reset state, so no need to set it)
    const MODER: *mut u32 = 0x4800_1000 as *mut u32;
    let mut moder_val = ptr::read_volatile(MODER);
    const MODER8_POSITION: u32 = 16;
    moder_val = (moder_val & !(0b11 << MODER8_POSITION)) | (0b01 << MODER8_POSITION);
    ptr::write_volatile(MODER, moder_val);

    // Set pin.
    // BSRR.BS8 = 1
    const BSRR: *mut u32 = 0x4800_1018 as *mut u32;    
    const BS8_POSITION: u32 = 8;
    ptr::write_volatile(BSRR, 1u32 << BS8_POSITION);

    // TODO: After this, finish writeup on build script vs config file.
    // TODO: Write up gdb commands used.

    loop {}
}

#[panic_handler]
#[link_section = ".text"]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}