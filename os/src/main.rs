// main.rs
// #![deny(missing_docs)]

#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_item;
mod loader;
mod mm;
mod sbi;
mod sync;
mod timer;

pub mod syscall;
pub mod task;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello Ros!");
    mm::init();
    println!("back to Ros!");
    // mm::heap_test();
    mm::remap_test();
    trap::init();
    // loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();

    panic!("Unreachable in rust_main!");
}
