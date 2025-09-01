#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3::stm32f303;

#[entry]
fn entry() -> ! {
    let peripherals = stm32f303::Peripherals::take().unwrap();
    init_led(&peripherals);
    init_delay(&peripherals);
    loop {
        toggle_led(&peripherals);
        delay_ms(&peripherals, 1000);
    }
}

fn init_led(peripherals: &stm32f303::Peripherals) {
    // Enable AHB clock for Port E pins.
    peripherals.RCC.ahbenr.modify(|_r, w| w.iopeen().enabled());

    // Configure Port E Pin 8 as a push-pull output.
    peripherals.GPIOE.moder.modify(|_r, w| w.moder8().variant(stm32f303::gpioc::moder::MODER0_A::OUTPUT));
}

fn toggle_led(peripherals: &stm32f303::Peripherals) {
    peripherals.GPIOE.odr.modify(|r, w| w.odr8().bit(!r.odr8().bit()));
}

fn init_delay(peripherals: &stm32f303::Peripherals) {
    // NOTE: APB1CLK is always enabled.

    // Divide APB1CLK by 16.
    // SYSCLK is driven by HSI, which runs at 8 MHz.
    // The desired TIM6CLK is 1 MHz, but if APB1CLK is divided at all, it's multiplied by 2 prior to setting TIM6CLK.
    // Therefore, divide APB1CLK by 16 to get a TIM6CLK of 1 MHz.
    peripherals.RCC.cfgr.modify(|_r, w| w.ppre1().div16());

    // Enable TIM6CLK.
    peripherals.RCC.apb1enr.modify(|_r, w| w.tim6en().enabled());
    
    // Set the auto-reload register to 1,000.
    // TIM6CLK runs at 1 MHz, which has a period of 1 microsecond. 1 millisecond has passed when the counter reaches 1,000.
    peripherals.TIM6.arr.write(|w| w.arr().bits(1000));
    
    // The UIF status bit is copied to TIM6_CNT(31).
    peripherals.TIM6.cr1.modify(|_r, w| w.uifremap().set_bit());
}

fn delay_ms(peripherals: &stm32f303::Peripherals, count_ms: u32) {
    // Reset the counter to 0.
    let tim6 = &peripherals.TIM6;
    tim6.cnt.modify(|_r, w| w.cnt().bits(0));
    
    // Enable the counter.
    tim6.cr1.modify(|_r, w| w.cen().enabled());

    for _ in 0..count_ms {
        // Wait for the update interrupt flag to be set.
        while tim6.cnt.read().uifcpy().bit_is_clear() {}
        
        // Clear the update interrupt flag.
        tim6.sr.modify(|_r, w| w.uif().clear());
    }

    // Disable the counter.
    tim6.cr1.modify(|_r, w| w.cen().disabled());
}