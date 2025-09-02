use std::fmt::Debug;
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::exit;
use std::string;
use smart_house_lib::power_socket::PowerSocketState;

trait Transport {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub enum FakePowerSocketState {
    OK,
    ERR,
    OFF,
    ON,
    EXIT,
}
pub struct FakePowerSocket {
    state: PowerSocketState,
    port: u16,
}

impl FakePowerSocket {
    fn new(port: u16) -> Self {
        Self {
            state: PowerSocketState::OFF,
            port,
        }
    }
    
    fn parse(buf: &[u8]) -> &str {
        let cmd  = match str::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => {
                return "ERR";
            }
        };
        cmd
    }

    fn pack(&self, state: &str, buf: &mut [u8]) -> &mut [u8] {
        for (i, c) in state.chars().enumerate() {
            buf[i] = c as u8;
        }
        &mut buf[0..state.len()]
    }

    fn process(&mut self, buf: &[u8]) -> FakePowerSocketState {
        match FakePowerSocket::parse(buf) {
            "state" => {
                match self.state {
                    PowerSocketState::ON => FakePowerSocketState::ON,
                    PowerSocketState::OFF => FakePowerSocketState::OFF,
                }
            },
            "off" => {
                self.state = PowerSocketState::OFF;
                FakePowerSocketState::OK
            }
            "on" => {
                self.state = PowerSocketState::ON;
                FakePowerSocketState::OK
            }
            "exit" => {
                FakePowerSocketState::EXIT
            }
            _ => {
                FakePowerSocketState::ERR
            }
        }
    }
}

impl Transport for FakePowerSocket {
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("FakePowerSocket running on port {}", self.port);

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(&mut async move {
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

                    match self.process(&buf[0..n]) {
                        FakePowerSocketState::EXIT => {
                            return;
                        }
                        state => {
                            buf[0] = state as u8;
                            if let Err(e) = socket.write_all(&buf[0..1]).await {
                                eprintln!("failed to write to socket; err = {:?}", e);
                                return;
                            }
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