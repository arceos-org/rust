use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::os::arceos::net::{AsRawTcpSocket, FromRawTcpSocket, IntoRawTcpSocket};
use crate::os::arceos::net::{AsRawUdpSocket, FromRawUdpSocket, IntoRawUdpSocket};
use crate::sys::{cvt, unsupported};
use crate::time::Duration;
use crate::vec::IntoIter;

use arceos_api::net::{self as api, AxTcpSocketHandle, AxUdpSocketHandle};

////////////////////////////////////////////////////////////////////////////////
// TCP streams
////////////////////////////////////////////////////////////////////////////////

pub struct TcpStream {
    inner: AxTcpSocketHandle,
}

impl TcpStream {
    pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        let addr = addr?;
        let sock = api::ax_tcp_socket();
        cvt(api::ax_tcp_connect(&sock, *addr))?;
        Ok(TcpStream { inner: sock })
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(api::ax_tcp_recv(&self.inner, buf))
    }

    pub fn read_buf(&self, _buf: BorrowedCursor<'_>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        cvt(api::ax_tcp_send(&self.inner, buf))
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_peer_addr(&self.inner))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_socket_addr(&self.inner))
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        cvt(api::ax_tcp_shutdown(&self.inner))
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        cvt(api::ax_tcp_set_nodelay(&self.inner, nodelay))
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        cvt(api::ax_tcp_nodelay(&self.inner))
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        cvt(api::ax_tcp_set_nonblocking(&self.inner, nonblocking))
    }
}

impl AsRawTcpSocket for TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> &AxTcpSocketHandle {
        &self.inner
    }
}

impl FromRawTcpSocket for TcpStream {
    #[inline]
    unsafe fn from_raw_socket(sock: AxTcpSocketHandle) -> TcpStream {
        TcpStream { inner: sock }
    }
}

impl IntoRawTcpSocket for TcpStream {
    #[inline]
    fn into_raw_socket(self) -> AxTcpSocketHandle {
        self.inner
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = f.debug_struct("TcpStream");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        if let Ok(peer) = self.peer_addr() {
            res.field("peer", &peer);
        }

        res.finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP listeners
////////////////////////////////////////////////////////////////////////////////

pub struct TcpListener {
    inner: AxTcpSocketHandle,
}

impl TcpListener {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        let addr = addr?;
        let backlog = 128;
        let sock = api::ax_tcp_socket();
        cvt(api::ax_tcp_bind(&sock, *addr))?;
        cvt(api::ax_tcp_listen(&sock, backlog))?;
        Ok(TcpListener { inner: sock })
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_socket_addr(&self.inner))
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (sock, addr) = cvt(api::ax_tcp_accept(&self.inner))?;
        Ok((TcpStream { inner: sock }, addr))
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        cvt(api::ax_tcp_set_nonblocking(&self.inner, nonblocking))
    }
}

impl AsRawTcpSocket for TcpListener {
    #[inline]
    fn as_raw_socket(&self) -> &AxTcpSocketHandle {
        &self.inner
    }
}

impl FromRawTcpSocket for TcpListener {
    #[inline]
    unsafe fn from_raw_socket(sock: AxTcpSocketHandle) -> TcpListener {
        TcpListener { inner: sock }
    }
}

impl IntoRawTcpSocket for TcpListener {
    #[inline]
    fn into_raw_socket(self) -> AxTcpSocketHandle {
        self.inner
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = f.debug_struct("TcpListener");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        res.finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// UDP
////////////////////////////////////////////////////////////////////////////////

pub struct UdpSocket {
    inner: AxUdpSocketHandle,
}

impl UdpSocket {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        let addr = addr?;
        let sock = api::ax_udp_socket();
        cvt(api::ax_udp_bind(&sock, *addr))?;
        Ok(UdpSocket { inner: sock })
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_udp_peer_addr(&self.inner))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_udp_socket_addr(&self.inner))
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        cvt(api::ax_udp_recv_from(&self.inner, buf))
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        cvt(api::ax_udp_peek_from(&self.inner, buf))
    }

    pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
        cvt(api::ax_udp_send_to(&self.inner, buf, *dst))
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        cvt(api::ax_udp_set_nonblocking(&self.inner, nonblocking))
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(api::ax_udp_recv(&self.inner, buf))
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        cvt(api::ax_udp_send(&self.inner, buf))
    }

    pub fn connect(&self, addr: io::Result<&SocketAddr>) -> io::Result<()> {
        let addr = addr?;
        cvt(api::ax_udp_connect(&self.inner, *addr))
    }
}

impl AsRawUdpSocket for UdpSocket {
    #[inline]
    fn as_raw_socket(&self) -> &AxUdpSocketHandle {
        &self.inner
    }
}

impl FromRawUdpSocket for UdpSocket {
    #[inline]
    unsafe fn from_raw_socket(sock: AxUdpSocketHandle) -> UdpSocket {
        UdpSocket { inner: sock }
    }
}

impl IntoRawUdpSocket for UdpSocket {
    #[inline]
    fn into_raw_socket(self) -> AxUdpSocketHandle {
        self.inner
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = f.debug_struct("UdpSocket");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        res.finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// get_host_addresses
////////////////////////////////////////////////////////////////////////////////

pub struct LookupHost {
    iter: IntoIter<IpAddr>,
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
        self.iter.next().map(|ip| SocketAddr::new(ip, self.port))
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

    fn try_from((host, port): (&'a str, u16)) -> io::Result<LookupHost> {
        let addrs = cvt(api::ax_dns_query(host))?;
        Ok(LookupHost { iter: addrs.into_iter(), port })
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
