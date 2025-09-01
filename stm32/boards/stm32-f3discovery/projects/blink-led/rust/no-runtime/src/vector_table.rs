use crate::runtime::{reset, default_handler};

#[link_section = ".vector_table.reset_vector"]
pub static RESET_HANDLER: unsafe extern "C" fn() -> ! = reset;

#[link_section = ".vector_table.exceptions"]
pub static EXCEPTIONS: [fn() -> !; 14] = [default_handler; 14];

#[link_section = ".vector_table.interrupts"]
pub static INTERRUPTS: [fn() -> !; 240] = [default_handler; 240];