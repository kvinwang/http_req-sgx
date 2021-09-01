use sgx_tstd::io::{Read, Write};
use sgx_tstd::net::TcpStream as SgxTcpStream;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

pub struct TcpStream(SgxTcpStream);


fn to_std(error: sgx_tstd::io::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{}", error))
}

impl TcpStream {
    pub fn connect(addr: (&str, u16)) -> io::Result<TcpStream> {
        Ok(Self(SgxTcpStream::connect(addr).map_err(to_std)?))
    }

    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
        let addr = match addr {
            SocketAddr::V4(addr) => {
                let ip = addr.ip();
                let port = addr.port();
                let ip = sgx_tstd::net::Ipv4Addr::from(ip.octets());
                sgx_tstd::net::SocketAddr::V4(sgx_tstd::net::SocketAddrV4::new(ip, port))
            }
            SocketAddr::V6(addr) => {
                let ip = addr.ip();
                let port = addr.port();
                let flowinfo = addr.flowinfo();
                let scope_id = addr.scope_id();
                let ip = sgx_tstd::net::Ipv6Addr::from(ip.octets());
                sgx_tstd::net::SocketAddr::V6(sgx_tstd::net::SocketAddrV6::new(ip, port, flowinfo, scope_id))
            }
        };
        Ok(Self(SgxTcpStream::connect_timeout(&addr, timeout).map_err(to_std)?))
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.0.set_read_timeout(dur).map_err(to_std)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.0.set_write_timeout(dur).map_err(to_std)
    }
}

impl io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf).map_err(to_std)
    }
}

impl io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf).map_err(to_std)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush().map_err(to_std)
    }
}
