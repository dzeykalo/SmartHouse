use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::io::AsyncReadExt;

pub trait Transport {
    fn send(&mut self, cmd: &str);
    fn receive(&mut self) -> String;
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
    fn send(&mut self, cmd: &str) {
        for _ in 0..1 {
            if let Some(stream) = &mut self.stream {
                stream.write_all(cmd.as_bytes()).unwrap();
            } else {
                self.stream = TcpStream::connect(&self.address).ok();
            }
        }
    }

    fn receive(&mut self) -> String {
        let mut result = String::new();
        if let Some(stream) = &mut self.stream {
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).unwrap_or_default();
            match std::str::from_utf8(&buf[..n]) {
                Ok(s) => result = s.to_string(),
                Err(_) => {}
            }
        }
        result
    }
}


pub struct UdpTransport {
    socket: std::net::UdpSocket,
}

impl UdpTransport {
    pub fn new(ip: &str, port: u16) -> Self {
        let socket = std::net::UdpSocket::bind(format!("{}:{}", ip, port)).expect(
            "couldn't bind to address",
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