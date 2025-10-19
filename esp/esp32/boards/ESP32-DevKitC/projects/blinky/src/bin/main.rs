#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::cell::RefCell;
use critical_section::Mutex;
use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, DriveStrength, Level, Output, OutputConfig, Pull};
use esp_hal::{main, handler, Blocking};
use esp_hal::time::Duration;
use esp_hal::timer::{timg::TimerGroup, PeriodicTimer};
use esp_hal::interrupt::Priority;
use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static LED: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<PeriodicTimer<'_, Blocking>>>> = Mutex::new(RefCell::new(None));

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Configure GPIO13 as the output pin.
    let out_pin_cfg = OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull)
        .with_pull(Pull::None)
        .with_drive_strength(DriveStrength::_10mA);
    // Initialize the pin to high.
    let led = Output::new(peripherals.GPIO13, Level::High, out_pin_cfg);
    critical_section::with(|cs| LED.borrow_ref_mut(cs).replace(led));

    // Start a 1-second periodic timer. Handle the timing elapsing with an interrupt.
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut timer = PeriodicTimer::new(timg0.timer0);
    timer.enable_interrupt(true);
    timer.set_interrupt_handler(timer_isr);
    critical_section::with(|cs| {
        // Inspecting the source, the only reason I can see this could fail is if an out-of-range duration is specified.
        timer.start(Duration::from_secs(1)).expect("failed to start the timer");
        TIMER.borrow_ref_mut(cs).replace(timer);
    });

    loop {}
}

#[handler(priority = Priority::max())]
fn timer_isr() {
    // Toggle LED.
    let mut led_on = false;
    critical_section::with(|cs| {
        let mut opt = LED.borrow_ref_mut(cs);
        let led = opt.as_mut().unwrap();
        led.toggle();
        led_on = led.is_set_high();
    });
    info!("LED is {}", match led_on {
        true => "ON",
        false => "OFF",
    });

    // Clear interrupt on timer.
    critical_section::with(|cs| {
        TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}
