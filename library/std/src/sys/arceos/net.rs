use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::time::Duration;
use crate::sys::arceos::abi;
use alloc::vec::IntoIter;

pub struct TcpStream {
    inner: Socket,
}

impl TcpStream {
    pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        let addr = addr?;

        init();

        let sock = Socket::new(addr, abi::SOCK_STREAM)?;

        unsafe { abi::sys_connect(sock.as_raw(), addr) };
        Ok(TcpStream { inner: sock })
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        panic!("TcpStream::connect_timeout");
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        panic!("TcpStream::set_read_timeout");
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        panic!("TcpStream::set_write_timeout");
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        panic!("TcpStream::read_timeout");
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        panic!("TcpStream::write_timeout");
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        panic!("TcpStream::peek");
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    pub fn read_buf(&self, _buf: BorrowedCursor<'_>) -> io::Result<()> {
        panic!("TcpStream::read_buf");
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        panic!("TcpStream::read_vectored");
    }

    pub fn is_read_vectored(&self) -> bool {
        panic!("TcpStream::is_read_vectored");
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        Ok(unsafe { abi::sys_send(self.inner.as_raw(), buf) })
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        panic!("TcpStream::write_vectored");
    }

    pub fn is_write_vectored(&self) -> bool {
        panic!("TcpStream::is_write_vectored");
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        panic!("TcpStream::peer_addr");
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        panic!("TcpStream::socket_addr");
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        panic!("TcpStream::shutdown");
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        panic!("TcpStream::duplicate");
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        panic!("TcpStream::set_linger");
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        panic!("TcpStream::linger");
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        panic!("TcpStream::set_nodelay");
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        panic!("TcpStream::nodelay");
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        panic!("TcpStream::set_ttl");
    }

    pub fn ttl(&self) -> io::Result<u32> {
        panic!("TcpStream::ttl");
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("TcpStream::take_error");
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        panic!("TcpStream::set_nonblocking");
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("TcpStream::fmt");
    }
}

pub struct TcpListener {
    inner: Socket,
}

impl TcpListener {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        let addr = addr?;

        init();

        let sock = Socket::new(addr, abi::SOCK_STREAM)?;

        // Todo: need setsockopt(s, SOL_SOCKET, SO_REUSEADDR) ???

        // Bind our new socket
        println!("before netc bind... {:?}", addr);
        unsafe { abi::sys_bind(sock.as_raw(), addr) };
        println!("after netc bind !");

        // Start listening, backlog is 128.
        unsafe { abi::sys_listen(sock.as_raw(), 128) };
        Ok(TcpListener { inner: sock })
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        abi_ret!(
            abi::sys_getsockname(self.inner.as_raw())
        )
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (handle, addr) = abi_ret!(
            abi::sys_accept(self.inner.as_raw())
        )?;
        let sock = Socket::new_from_handle(handle);
        Ok((TcpStream { inner: sock }, addr))
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        panic!("sys::arceos::net::duplicate");
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        panic!("sys::arceos::net::set_ttl");
    }

    pub fn ttl(&self) -> io::Result<u32> {
        panic!("sys::arceos::net::ttl");
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        panic!("sys::arceos::net::set_only_v6");
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        panic!("sys::arceos::net::only_v6");
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("sys::arceos::net::take_error");
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        panic!("sys::arceos::net::set_nonblocking");
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("sys::arceos::net::TcpListener::fmt");
    }
}

pub struct UdpSocket {
    inner: Socket,
}

impl UdpSocket {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        let addr = addr?;

        init();

        let sock = Socket::new(addr, abi::SOCK_DGRAM)?;

        unsafe { abi::sys_bind(sock.as_raw(), addr) };

        Ok(UdpSocket { inner: sock })
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        panic!("UdpSocket::peer_addr");
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        abi_ret!(
            abi::sys_getsockname(self.inner.as_raw())
        )
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.recv_from(buf)
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        panic!("UdpSocket::...");
    }

    pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
        unsafe {
            Ok(abi::sys_sendto(self.inner.as_raw(), buf, dst))
        }
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        panic!("UdpSocket");
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        panic!("UdpSocket");
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        panic!("UdpSocket");
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        panic!("UdpSocket");
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        panic!("UdpSocket");
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        panic!("UdpSocket");
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        panic!("UdpSocket");
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn ttl(&self) -> io::Result<u32> {
        panic!("UdpSocket");
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("UdpSocket");
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        panic!("UdpSocket");
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        panic!("UdpSocket");
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        panic!("UdpSocket");
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        panic!("UdpSocket");
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        panic!("UdpSocket");
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("UdpSocket::fmt");
    }
}

pub struct LookupHost {
    iter: IntoIter<SocketAddr>,
    port: u16,
}

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.iter.next()
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(s: &str) -> io::Result<LookupHost> {
        macro_rules! try_opt {
            ($e:expr, $msg:expr) => {
                match $e {
                    Some(r) => r,
                    None => return Err(io::const_io_error!(io::ErrorKind::InvalidInput, $msg)),
                }
            };
        }

        // split the string by ':' and convert the second part to u16
        let (host, port_str) = try_opt!(s.rsplit_once(':'), "invalid socket address");
        let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");
        (host, port).try_into()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(v: (&'a str, u16)) -> io::Result<LookupHost> {
        let (name, port) = v;
        let iter = abi_ret!(
            abi::sys_getaddrinfo(name, port)
        )?.into_iter();
        Ok(LookupHost { iter, port })
    }
}

#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_INET: u8 = 0;
    pub const AF_INET6: u8 = 1;
    pub type sa_family_t = u8;

    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: u16,
        pub sin_addr: in_addr,
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr {}
}

/// Checks whether the socket interface has been started already, and
/// if not, starts it.
pub fn init() {
}

#[derive(Debug)]
pub struct Socket(usize);

impl Socket {
    pub fn new(addr: &SocketAddr, ty: i32) -> io::Result<Socket> {
        println!("Socket::new ...");
        let fam = match *addr {
            SocketAddr::V4(..) => netc::AF_INET,
            SocketAddr::V6(..) => netc::AF_INET6,
        };
        Socket::new_raw(fam as i32, ty)
    }

    pub fn new_raw(fam: i32, ty: i32) -> io::Result<Socket> {
        let handle = unsafe { abi::sys_socket(fam, ty) };
        Ok(Socket(handle))
    }

    pub fn as_raw(&self) -> usize {
        self.0
    }

    pub fn new_from_handle(handle: usize) -> Socket {
        Socket(handle)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_with_flags(buf, 0)
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: i32) -> io::Result<usize> {
        unsafe {
            Ok(abi::sys_recv(self.as_raw(), buf, flags))
        }
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.recv_from_with_flags(buf, 0)
    }

    fn recv_from_with_flags(&self, buf: &mut [u8], flags: i32)
        -> io::Result<(usize, SocketAddr)>
    {
        unsafe {
            Ok(abi::sys_recvfrom(self.as_raw(), buf, flags))
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { abi::sys_close_socket(self.0) }
    }
}
