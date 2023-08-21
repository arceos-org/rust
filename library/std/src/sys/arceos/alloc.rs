use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr::NonNull;

use arceos_api::mem as api;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        api::ax_alloc(layout).unwrap().as_ptr()
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        api::ax_dealloc(NonNull::new(ptr).unwrap(), layout)
    }
}
