//! System bindings for ArceOS
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for ArceOS.
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for wasm. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

#![allow(missing_docs, nonstandard_style, unsafe_op_in_unsafe_fn)]

use crate::os::raw::c_char;
use axbase::LinuxError;

macro_rules! abi_ret {
    ($expr:expr) => {
        unsafe {
            $expr.or_else(|e| {
                Err(io::Error::new(io::ErrorKind::Other, e.as_str()))
            })
        }
    };
}

#[path = "../unsupported/once.rs"]
pub mod once;

pub mod net;

pub mod thread;
#[path = "../unsupported/thread_parking.rs"]
pub mod thread_parking;

#[path = "../unix/cmath.rs"]
pub mod cmath;

pub mod fs;
pub mod alloc;
pub mod args;
pub mod futex;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod os;
pub mod memchr;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
pub mod time;

#[path = "../unix/locks"]
pub mod locks {
    mod futex_condvar;
    mod futex_mutex;
    mod futex_rwlock;
    pub(crate) use futex_condvar::Condvar;
    pub(crate) use futex_mutex::Mutex;
    pub(crate) use futex_rwlock::RwLock;
}

use crate::io::ErrorKind;
use crate::os::arceos::abi;

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::const_io_error!(
        crate::io::ErrorKind::Unsupported,
        "operation not supported on ArceOS yet",
    )
}

pub fn abort_internal() -> ! {
    unsafe { abi::sys_terminate(); }
}

// FIXME: just a workaround to test the system
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8, _sigpipe: u8) {
    let _ = net::init();
    args::init(argc, argv);
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern "C" fn runtime_entry(
    argc: i32,
    argv: *const *const c_char,
    env: *const *const c_char,
) {
    use crate::sys::arceos::thread_local_dtor::run_dtors;
    extern "C" {
        fn main(argc: isize, argv: *const *const c_char) -> i32;
    }

    // initialize environment
    os::init_environment(env as *const *const i8);

    main(argc as isize, argv);

    run_dtors();
}

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    let err = match errno.try_into() {
        Ok(e) => e,
        Err(_) => {
            return ErrorKind::Uncategorized
        }
    };
    match err {
        LinuxError::EACCES => ErrorKind::PermissionDenied,
        LinuxError::EADDRINUSE => ErrorKind::AddrInUse,
        LinuxError::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        LinuxError::EAGAIN => ErrorKind::WouldBlock,
        LinuxError::ECONNABORTED => ErrorKind::ConnectionAborted,
        LinuxError::ECONNREFUSED => ErrorKind::ConnectionRefused,
        LinuxError::ECONNRESET => ErrorKind::ConnectionReset,
        LinuxError::EEXIST => ErrorKind::AlreadyExists,
        LinuxError::EINTR => ErrorKind::Interrupted,
        LinuxError::EINVAL => ErrorKind::InvalidInput,
        LinuxError::ENOENT => ErrorKind::NotFound,
        LinuxError::ENOTCONN => ErrorKind::NotConnected,
        LinuxError::EPERM => ErrorKind::PermissionDenied,
        LinuxError::EPIPE => ErrorKind::BrokenPipe,
        LinuxError::ETIMEDOUT => ErrorKind::TimedOut,
        _ => ErrorKind::Uncategorized,
    }
}
