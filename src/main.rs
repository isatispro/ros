// os/src/main.rs
#![no_std]
#![no_main]
#![feature(panic_info_message)]

// use log::*;

#[macro_use]
mod console;
mod lang_item;
// mod logging;
mod sbi;

// use core::{arch::global_asm, panicking::panic};

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello Ros!");
    panic!("shutdown Ros!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
