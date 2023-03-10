use std::io::{Result, Write};
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(not(tarpaulin_include))]
pub struct TcpWriter {
    pub stream: TcpStream,
}

#[cfg(not(tarpaulin_include))]
impl TcpWriter {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }
}

#[cfg(not(tarpaulin_include))]
impl Clone for TcpWriter {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.try_clone().unwrap(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl Write for TcpWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}
