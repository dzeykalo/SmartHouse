use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait PowerSocketTransport {
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

impl PowerSocketTransport for TcpTransport {
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

pub trait ThermometerTransport {
    fn get_value(&mut self) -> f64;
    fn receiver(&mut self);
}
pub struct UdpTransport {
    socket: std::net::UdpSocket,
    data: Arc<Mutex<f64>>,
}

impl UdpTransport {
    pub fn new(ip: &str, port: u16) -> Self {
        let socket = std::net::UdpSocket::bind(format!("{}:{}", ip, port)).expect(
            "could not bind to socket",
        );
        Self {
            socket,
            data: Arc::new(Mutex::new(0.0)),
        }
    }
}

impl ThermometerTransport for UdpTransport {
    fn get_value(&self) -> f64 {
        *self.data.lock().unwrap()
    }
    fn receiver(&mut self) -> String {
        let mut buf = [0; 1024];
        let data_clone = Arc::clone(&self.data);
        loop {
            // Получаем данные от клиента
            match self.socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("Получено {} байт от {}: {:?}", amt, src, &buf[..amt]);
                    let mut num = data_clone.lock().unwrap();
                    *num = buf[..amt].iter().collect::<String>().parse().unwrap_or_else(|e| {
                        eprintln!("Error parsing number: {}", e);
                        *num
                    })
                },
                Err(e) => {
                    eprintln!("Receive error: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}

pub struct MockTransport {
    state: String,
}

impl MockTransport {
    pub fn new() -> Self {
        Self { state: "".to_string() }
    }
}

impl PowerSocketTransport for MockTransport {
    fn send(&mut self, cmd: &str) {
        match cmd {
            "on" => self.state = "on".to_string(),
            "off" => self.state = "off".to_string(),
            _ => {}
        }
    }

    fn receive(&mut self) -> String {
        self.state.to_string()
    }
}