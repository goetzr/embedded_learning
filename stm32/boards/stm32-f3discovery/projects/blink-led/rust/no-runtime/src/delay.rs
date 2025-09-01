use core::ptr;

const RCC_BASE: u32 = 0x40021000;
const RCC_CFGR: *mut u32 = (RCC_BASE + 0x04) as *mut u32;
const PPRE1: u32 = 8;
const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x1C) as *mut u32;
const TIM6EN: u32 = 4;
const TIM6_BASE: u32 = 0x40001000;
const TIM6_CR1: *mut u32 = (TIM6_BASE + 0x00) as *mut u32;
const UIFREMAP: u32 = 11;
const CEN: u32 = 0;
const TIM6_SR: *mut u32 = (TIM6_BASE + 0x10) as *mut u32;
const UIF: u32 = 0;
const TIM6_CNT: *mut u32 = (TIM6_BASE + 0x24) as *mut u32;
const UIFCPY: u32 = 31;
const TIM6_ARR: *mut u32 = (TIM6_BASE + 0x2C) as *mut u32;

#[link_section = ".text.delay"]
pub unsafe fn init_delay() {
    // NOTE: APB1CLK is always enabled.

    // Divide APB1CLK by 16 (RCC_CFGR.PPRE1 = 111).
    // SYSCLK is driven by HSI, which runs at 8 MHz.
    // The desired TIM6CLK is 1 MHz, but if APB1CLK is divided at all, it's multiplied by 2 prior to setting TIM6CLK.
    // Therefore, divide APB1CLK by 16 to get a TIM6CLK of 1 MHz.
    let mut val: u32 = ptr::read_volatile(RCC_CFGR);
    val |= 0b111 << PPRE1;
    ptr::write_volatile(RCC_CFGR, val);

    // Enable TIM6CLK.
    val = ptr::read_volatile(RCC_APB1ENR);
    val |= 1 << TIM6EN;
    ptr::write_volatile(RCC_APB1ENR, val);
    
    // Set the auto-reload register to 1,000.
    // TIM6CLK runs at 1 MHz, which has a period of 1 microsecond. 1 millisecond has passed when the counter reaches 1,000.
    ptr::write_volatile(TIM6_ARR, 1000);
    
    // The UIF status bit is copied to TIM6_CNT(31).
    val = ptr::read_volatile(TIM6_CR1);
    val |= 1 << UIFREMAP;
    ptr::write_volatile(TIM6_CR1, val);
}

#[link_section = ".text.delay"]
pub unsafe fn delay_ms(num_ms: u32) {
    // Reset the counter to 0.
    ptr::write_volatile(TIM6_CNT, 0);
    
    // Enable the counter.
    let mut val: u32 = ptr::read_volatile(TIM6_CR1);
    val |= 1 << CEN;
    ptr::write_volatile(TIM6_CR1, val);

    for _ in 0..num_ms {
        // Wait for the update interrupt flag to be set.
        while ptr::read_volatile(TIM6_CNT) & 1 << UIFCPY == 0 {}
        
        // Clear the update interrupt flag.
        val = ptr::read_volatile(TIM6_SR);
        val &= !(1 << UIF);
        ptr::write_volatile(TIM6_SR, val);
    }

    // Disable the counter.
    val = ptr::read_volatile(TIM6_CR1);
    val &= !(1 << CEN);
    ptr::write_volatile(TIM6_CR1, val);
}