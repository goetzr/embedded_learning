#![no_main]
#![no_std]

// Some panic handler needs to be included.
use panic_halt as _;

use cortex_m_rt::entry;

mod led;
mod delay;

use led::{init_led, set_led, LedState};
use delay::{init_delay, delay_ms};

#[entry]
unsafe fn main() -> ! {
    init_led();
    init_delay();
    let mut state = LedState::On;

    loop {
        set_led(state);
        delay_ms(1000);
        state.toggle();
    }
}