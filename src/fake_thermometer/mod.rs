use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::UdpSocket;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    interval_ms: u64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config-file.json>", args[0]);
        exit(1);
    }

    let config_file_path = args[1].clone();
    let data = match fs::read_to_string(config_file_path.clone()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", config_file_path, e);
            exit(1);
        }
    };

    let cfg: Config = match serde_json::from_str(&data) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing JSON file '{}': {}", config_file_path, e);
            exit(1);
        }
    };

    let server_address = format!("{}:{}", cfg.host, cfg.port);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");
    loop {
        let random_temperature: f32 = rand::rng().random_range(18.0..=26.0);
        let message = format!("{:.2}", random_temperature);
        socket
            .send_to(message.as_bytes(), &server_address)
            .expect("Error send to thermometer server");
        sleep(Duration::from_millis(cfg.interval_ms));
    }
}
