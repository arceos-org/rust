use crate::io;
use crate::io::{IoSlice, IoSliceMut};
use crate::sys::arceos::abi;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        unsafe { Ok(abi::sys_console_read_bytes(data)) }
    }

    fn read_vectored(&mut self, data: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsafe {
            let (_, data, _) = data.align_to_mut::<u8>();
            Ok(abi::sys_console_read_bytes(data))
        }
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        true
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        unsafe {
            abi::sys_console_write_bytes(data);
        }
        Ok(data.len() as usize)
    }

    fn write_vectored(&mut self, data: &[IoSlice<'_>]) -> io::Result<usize> {
        unsafe {
            let (_, data, _) = data.align_to::<u8>();
            abi::sys_console_write_bytes(data);
            Ok(data.len() as usize)
        }
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
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
        unsafe {
            abi::sys_console_write_bytes(data);
        }
        Ok(data.len() as usize)
    }

    fn write_vectored(&mut self, data: &[IoSlice<'_>]) -> io::Result<usize> {
        // Todo: implement specific stderr in arceos.
        unsafe {
            let (_, data, _) = data.align_to::<u8>();
            abi::sys_console_write_bytes(data);
            Ok(data.len() as usize)
        }
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
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
