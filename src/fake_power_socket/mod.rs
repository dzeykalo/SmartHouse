use std::fmt::Debug;
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::exit;
use std::string;
use std::sync::{Arc, Mutex, MutexGuard};
use smart_house_lib::power_socket::PowerSocketState;

trait Transport {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}


pub struct FakePowerSocket {
    state: Arc<Mutex<PowerSocketState>>,
    port: u16,
}

impl FakePowerSocket {
    fn new(port: u16) -> Self {
        Self {
            state: Arc::new(Mutex::new(PowerSocketState::OFF)),
            port,
        }
    }
    
    fn parse(buf: &[u8]) -> &str {
        let cmd  = match str::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed to parse command from socket; buf = {:?}", buf);
                return "error";
            }
        };
        cmd.trim()
    }

    fn process(buf: &[u8], state: &mut PowerSocketState) -> String {
        let cmd = match FakePowerSocket::parse(buf) {
            "state" => {
                match *state {
                    PowerSocketState::ON => "on",
                    PowerSocketState::OFF => "off"
                }
            },
            "off" => {
                *state = PowerSocketState::OFF;
                "ok"
            }
            "on" => {
                *state = PowerSocketState::ON;
                "ok"
            }
            "exit" => {
                "bye"
            }
            "" => {
                ""
            }
            _ => {
               "error"
            }
        };
        cmd.to_string()
    }
}

impl Transport for FakePowerSocket {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("FakePowerSocket running on port {}", self.port);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let state_clone = Arc::clone(&self.state);

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    
                    let result = FakePowerSocket::process(&mut buf[0..n], 
                                                          &mut state_clone.lock().unwrap());
                    if !result.is_empty() {
                        if let Err(e) = socket.write_all(result.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            });
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Not found port!");
        exit(1);
    }

    let port = args[0].parse::<u16>();
    if port.is_err() {
        eprintln!("Failed to parse port");
        exit(2);
    }

    let mut fake_socket = FakePowerSocket::new(port.unwrap());
    fake_socket.run().await;
}
