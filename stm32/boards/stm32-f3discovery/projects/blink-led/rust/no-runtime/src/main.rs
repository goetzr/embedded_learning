#![no_main]
#![no_std]

use core::panic::PanicInfo;

pub mod led;
pub mod delay;
pub mod runtime;
pub mod main_func;
pub mod vector_table;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On a panic, loop forever
    loop {
        continue;
    }
}