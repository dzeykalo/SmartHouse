use crate::device::Device;
use crate::transport::Transport;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;
use crate::report::Report;

#[derive(Debug, Default)]
enum ThermometerState {
    #[default]
    On,
    Off,
}

#[derive(Debug, Default)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    // handle: Option<thread::JoinHandle<()>>,
    alive: Arc<atomic::AtomicBool>,
    state: ThermometerState,
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        self.alive.store(false, atomic::Ordering::Relaxed);
    }
}

impl Device for Thermometer {
    fn new(transport: Box<dyn Transport + Send>, t: f64) -> Self {
        let temperature = Arc::new(Mutex::new(t));
        let alive = Arc::new(atomic::AtomicBool::new(true));

        let temp_clone = Arc::clone(&temperature);
        let alive_clone = Arc::clone(&alive);
        thread::spawn(move || {
            let t = RefCell::new(transport);
            let mut tt = t.borrow_mut();

            while alive_clone.load(atomic::Ordering::Relaxed) {
                let s = tt.communicate("");
                let temperature = s.parse().unwrap_or_else(|e| {
                    eprintln!("Error parsing number: {}", e);
                    0.0f64
                });
                if temperature != 0.0f64 {
                    let mut num = temp_clone.lock().unwrap();
                    *num = temperature;
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        Self {
            temperature,
            alive,
            state: ThermometerState::On,
        }
    }

    fn get_value(&self) -> f64 {
        match self.state {
            ThermometerState::On => *self.temperature.lock().unwrap(),
            ThermometerState::Off => 0.0f64,
        }
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn get_state(&self) -> String {
        match self.state {
            ThermometerState::On => String::from("ON"),
            ThermometerState::Off => String::from("OFF"),
        }
    }

    fn on(&mut self) {
        self.state = ThermometerState::On;
    }

    fn off(&mut self) {
        self.state = ThermometerState::Off;
    }
}

impl Report for Thermometer {
    fn report(&self) -> String {
        format!("Thermometer state: {}, temperature: {}", self.get_state(), self.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    #[test]
    fn test_thermometer_get_value() {
        let thermometer =
            Thermometer::new(Box::new(MockTransport::new("25.0".to_string())), 0.0f64);
        thread::sleep(Duration::from_secs(1));
        assert_eq!(thermometer.get_value(), 25.0);
    }

    #[test]
    fn test_thermometer_off() {
        let mut thermometer =
            Thermometer::new(Box::new(MockTransport::new("25.0".to_string())), 0.0f64);
        thread::sleep(Duration::from_secs(1));
        thermometer.off();
        assert_eq!(thermometer.get_value(), 0.0);
    }
}
