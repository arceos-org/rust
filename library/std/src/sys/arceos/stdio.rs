use crate::io;
use crate::sys::cvt;

use arceos_api::stdio as api;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    // Non-blocking read, returns number of bytes read.
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        let mut read_len = 0;
        while read_len < data.len() {
            if let Some(c) = api::ax_console_read_byte() {
                data[read_len] = c;
                read_len += 1;
            } else {
                break;
            }
        }
        Ok(read_len)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        false
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        cvt(api::ax_console_write_bytes(data))
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        false
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        // Todo: implement specific stderr in arceos.
        cvt(api::ax_console_write_bytes(data))
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        false
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
