use std::fmt::Debug;
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::exit;
use std::string;
use smart_house_lib::power_socket::PowerSocketState;

trait Transport {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
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
    
    fn parse(&mut self, buf: &[u8]) -> &str {
        match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => "",
        }
    }

    fn cmd(&mut self, cmd: &str) -> String {
        match cmd {
            "state" => {
                let state = match self.state {
                    PowerSocketState::ON => "ON",
                    PowerSocketState::OFF => "OFF",
                };
                format!("{}", state)
            },
            "OFF" => {
                self.state = PowerSocketState::OFF;
                "OK".to_string()
            }
            "ON" => {
                self.state = PowerSocketState::ON;
                "OK".to_string()
            }
            _ => {
                "ERR".to_string()
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

                    self.cmd(self.parse(&buf[0..n]));
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
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

    let socket = FakePowerSocket::new(port.unwrap());
    socket.run().await;
}