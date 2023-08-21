#![allow(exported_private_dependencies)]
#![allow(dead_code)]

use crate::sys::fs::DirEntry;
use crate::sys::fs::FileAttr;
use core::sync::atomic::AtomicU32;
use core::time::Duration;

use axerrno::AxError;

extern "Rust" {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_futex_wake(futex: &AtomicU32, count: i32);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read_dir(path: &str) -> usize;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read_dir_next(handle: usize) -> Option<Result<DirEntry, AxError>>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_mkdir(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_stat(path: &str) -> Result<FileAttr, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_open(path: &str, opts: u32) -> Result<usize, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_write(handle: usize, buf: &[u8]) -> usize;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read(handle: usize, buf: &mut [u8]) -> usize;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_rmdir(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_unlink(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_file(handle: usize);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_dir(handle: usize);
}
