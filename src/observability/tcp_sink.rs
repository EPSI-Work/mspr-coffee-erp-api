use std::io::{Result, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;

pub struct TcpWriter {
    pub stream: TcpStream,
}

impl TcpWriter {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }
}

impl Clone for TcpWriter {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.try_clone().unwrap(),
        }
    }
}

impl Write for TcpWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn test_tcp_client_server() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server_thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();
            assert_eq!(String::from_utf8_lossy(&buffer[..bytes_read]), "hello");
        });

        let client = TcpWriter::new(format!("127.0.0.1:{}", port)).unwrap();
        let mut client = client.clone();
        client.write(b"hello").unwrap();
        client.flush().unwrap();

        server_thread.join().unwrap();
    }
}
