#![allow(dead_code)]

use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;
use crate::sys::arceos::abi;
use crate::sys::arceos::thread_local_dtor::run_dtors;

pub const LOW_PRIO:     i32 = 1;
pub const NORMAL_PRIO:  i32 = 2;
pub const HIGH_PRIO:    i32 = 3;

pub struct Thread {
    handle: usize,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = 1 << 20;

impl Thread {
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        Thread::new_with_coreid(stack, p, -1 /* = no specific core */)
    }

    pub unsafe fn new_with_coreid(
        stack: usize,
        p: Box<dyn FnOnce()>,
        core_id: isize,
    ) -> io::Result<Thread> {
        let thread_start = move|| {
            unsafe {
                p();

                // run all destructors
                run_dtors();
            }
        };

        let handle = abi::sys_spawn2(
            Box::new(thread_start),
            NORMAL_PRIO,
            stack,
            core_id,
        );

        Ok(Thread { handle: handle })
    }

    pub fn yield_now() {
        unsafe { abi::sys_yield_now(); }
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(dur: Duration) {
        unsafe { abi::sys_sleep(dur); }
    }

    pub fn join(self) {
        unsafe {
            let _ = abi::sys_join(self.handle);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { abi::sys_close_thread(self.handle) }
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
