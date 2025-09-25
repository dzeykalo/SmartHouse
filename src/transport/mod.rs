use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait Transport {
    fn send(&mut self, cmd: &str);
    fn receive(&mut self) -> String;
}

pub struct TcpTransport {
    stream: TcpStream,
}

impl TcpTransport {
    pub fn new(ip: &str, port: u16) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
        Self {
            stream,
        }
    }
}

impl Transport for TcpTransport {
    fn send(&mut self, cmd: &str) {
        self.stream.write_all(cmd.as_bytes()).unwrap();
    }

    fn receive(&mut self) -> String {
        let mut buf = [0; 1024];
        match self.stream.read(&mut buf) {
            Ok(n) => match std::str::from_utf8(&buf[..n]) {
                Ok(s) => s.to_string(),
                Err(_) => String::new(),
            },
            Err(_) => String::new(),
        }
    }
}


pub struct UdpTransport {
    socket: std::net::UdpSocket,
}

impl UdpTransport {
    pub fn new(ip: &str, port: u16) -> Self {
        let socket = std::net::UdpSocket::bind(format!("{}:{}", ip, port)).expect(
            "could not bind to socket",
        );
        Self {
            socket,
        }
    }
}

impl Transport for UdpTransport {
    fn send(&mut self, cmd: &str) {
        todo!()
    }
    fn receive(&mut self) -> String {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((n, src)) => {
                println!("Получено {} байт от {}: {:?}", n, src, &buf[..n]);
                match std::str::from_utf8(&buf[..n]) {
                    Ok(s) => s.to_string(),
                    Err(_) => String::new(),
                }
            },
            Err(e) => {
                eprintln!("Receive error: {}", e);
                String::new()
            }
        }
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
    fn send(&mut self, cmd: &str) {
        match cmd {
            "on" => self.value = "on".to_string(),
            "off" => self.value = "off".to_string(),
            _ => {}
        }
    }

    fn receive(&mut self) -> String {
        self.value.clone()
    }
}