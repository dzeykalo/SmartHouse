use std::io::prelude::*;
use std::net::TcpStream;

pub trait Transport {
    fn communicate(&mut self, cmd: &str) -> String;
}

pub struct TcpTransport {
    address: String,
    stream: Option<TcpStream>,
}

impl TcpTransport {
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&address);
        Self {
            address,
            stream: stream.ok(),
        }
    }
}

impl Transport for TcpTransport {
    fn communicate(&mut self, cmd: &str) -> String {
        if self.stream.is_none() {
            self.stream = TcpStream::connect(&self.address).ok();
        }
        if let Some(stream) = &mut self.stream
            && stream.write_all(cmd.as_bytes()).is_ok()
        {
            let mut buf = [0; 1024];
            if let Ok(n) = stream.read(&mut buf)
                && let Ok(s) = std::str::from_utf8(&buf[..n])
                && !s.is_empty()
            {
                return s.to_string();
            }
        }
        self.stream = None;
        "disconnected".to_string()
    }
}

pub struct UdpTransport {
    socket: std::net::UdpSocket,
}

impl UdpTransport {
    pub fn new(ip: &str, port: u16) -> Self {
        let socket = std::net::UdpSocket::bind(format!("{}:{}", ip, port))
            .expect("couldn't bind to address");
        Self { socket }
    }
}

impl Transport for UdpTransport {
    fn communicate(&mut self, _cmd: &str) -> String {
        let mut buf = [0; 1024];
        if let Ok((n, _)) = self.socket.recv_from(&mut buf)
            && let Ok(s) = std::str::from_utf8(&buf[..n])
        {
            return s.to_string();
        }
        String::new()
    }
}

pub struct MockTransport {
    value: String,
}

impl MockTransport {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Transport for MockTransport {
    fn communicate(&mut self, cmd: &str) -> String {
        match cmd {
            "on" => self.value = "on".to_string(),
            "off" => self.value = "off".to_string(),
            _ => {}
        }
        self.value.to_string()
    }
}
