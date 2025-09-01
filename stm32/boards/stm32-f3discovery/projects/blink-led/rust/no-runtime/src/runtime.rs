use core::ptr;
use crate::main_func::main;

// NOTE: These are symbols defined in the linker script, not variables.
//       To get a symbol's value, its address must be taken.
extern "C" {
    static mut __sdata: u8;
    static mut __edata: u8;
    static mut __sidata: u8;
}

#[link_section = ".text.runtime"]
#[export_name = "Reset"]
pub unsafe extern "C" fn reset() -> ! {
    // Initialize SRAM.
    let mut sdata: *mut u8 = &mut __sdata;
    let edata: *const u8 = &__edata;
    let mut sidata: *const u8 = &__sidata;
    while sdata as *const u8 != edata {
        let data = ptr::read_volatile(sidata);
        ptr::write_volatile(sdata, data);
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }

    // Call main, which never returns.
    main();

}

#[link_section = ".text.runtime"]
pub fn default_handler() -> !{
    loop {}
}