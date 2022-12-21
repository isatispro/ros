//! App management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("[kernel] Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    // println!("[kernel] Application yield");
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    // println!("[kernel] Application get time");
    get_time_ms() as isize
}
