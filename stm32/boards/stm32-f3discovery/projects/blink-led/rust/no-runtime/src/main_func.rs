use crate::led::{LedState, init_led, set_led};
use crate::delay::{init_delay, delay_ms};

#[link_section = ".data.main"]
static mut LED_STATE: LedState = LedState::On;

#[link_section = ".text.main"]
#[export_name = "main"]
pub fn main() -> ! {
    unsafe {
        init_led();
        init_delay();

        loop {
            set_led(LED_STATE);
            delay_ms(1000);
            LED_STATE.toggle();
        }
    }
}