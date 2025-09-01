use core::ptr;

const RCC_AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
const IOPEEN: u32 = 21;
const GPIOE_MODER: *mut u32 = 0x4800_1000 as *mut u32;
const MODER8: u32 = 16;
const GPIOE_BSRR: *mut u32 = 0x4800_1018 as *mut u32;
const BS8: u32 = 8;
const BR8: u32 = 24;

#[link_section = ".text.led"]
pub unsafe fn init_led() {
    // Enable AHB clock for Port E pins.
    let mut val = ptr::read_volatile(RCC_AHBENR);
    val |= 1 << IOPEEN;
    ptr::write_volatile(RCC_AHBENR, val);

    // Configure Port E Pin 8 as a push-pull output.
    val = ptr::read_volatile(GPIOE_MODER);
    val |= 1 << MODER8;
    ptr::write_volatile(GPIOE_MODER, val);
}

#[derive(Copy, Clone)]
pub enum LedState {
    On, Off
}

impl LedState {
    #[link_section = ".text.led"]
    pub fn toggle(&mut self) {
        *self = match self {
            LedState::On => LedState::Off,
            LedState::Off => LedState::On,
        }
    }
}

#[link_section = ".text.led"]
pub unsafe fn set_led(state: LedState) {
    match state {
        LedState::On => ptr::write_volatile(GPIOE_BSRR, 1 << BS8),
        LedState::Off => ptr::write_volatile(GPIOE_BSRR, 1 << BR8),
    }
}