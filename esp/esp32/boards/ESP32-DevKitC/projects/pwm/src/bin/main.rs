#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::ledc::{Ledc, LSGlobalClkSource, LowSpeed, timer::{self, TimerIFace}};
use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.6.0

    // The ESP32 only supports CPU operations at 80/160/240 MHz -- all derived from PLL_CLK.
    // Running directly from the 40 MHz XTAL is not supported.
    // The 40 MHz XTAL is multiplied up to the desired CPU frequency.
    // The APB_CLK is 80 MHz when CPU_CLK source is PLL_CLK.
    // The REF_TICK is 1 MHz when CPU_CLK source is PLL_CLK (and APB_CLK is 80 MHz).
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    // This enables then resets the LEDC clock using the DPORT_PERIP_CLK_EN_REG and DPORT_PERIP_RST_EN_REG registers.
    let mut ledc = Ledc::new(peripherals.LEDC);
    
    // l kHz = 8E6 / (8 * 2^20)
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0.configure(timer::config::Config {
        duty: timer::config::Duty::Duty20Bit,
        clock_source: timer::LSClockSource::
        frequency: Rate::from_khz(24),
    })?;

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
