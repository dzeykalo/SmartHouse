use std::net::UdpSocket;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");
    let server_address = "127.0.0.1:8080";
    loop {
        let message = "23.2";
        socket.send_to(message.as_bytes(), server_address).expect("Error send to thermometer server");
        sleep(Duration::from_secs(1));
    }
}