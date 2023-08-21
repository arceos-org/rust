use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::sys::arceos::thread_local_dtor::run_dtors;
use crate::time::Duration;

use arceos_api::task::{self as api, AxTaskHandle};

pub struct Thread(AxTaskHandle);

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = arceos_api::config::TASK_STACK_SIZE;

impl Thread {
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let main = Box::into_raw(Box::new(p)).expose_addr();
        let thread_start = move || {
            Box::from_raw(crate::ptr::from_exposed_addr::<Box<dyn FnOnce()>>(main).cast_mut())();
            run_dtors();
        };
        let handle = api::ax_spawn(thread_start, "".to_string(), stack);
        Ok(Thread(handle))
    }

    pub fn yield_now() {
        api::ax_yield_now();
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(dur: Duration) {
        api::ax_sleep_until(arceos_api::time::ax_current_time() + dur);
    }

    pub fn join(self) {
        api::ax_wait_for_exit(self.0);
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    Ok(NonZeroUsize::new(arceos_api::config::SMP).unwrap())
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
