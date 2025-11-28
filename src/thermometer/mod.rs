use std::cell::RefCell;
use std::sync::{atomic, Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::device::Device;
use crate::transport::Transport;

// #[derive(Debug)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    handle: thread::JoinHandle<()>,
    alive: Arc<atomic::AtomicBool>
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        self.alive.store(false, atomic::Ordering::Relaxed);
    }
}

impl Device for Thermometer {
    fn new(transport: Box<dyn Transport + Send>) -> Self {
        let temperature = Arc::new(Mutex::new(0.0));
        let t = RefCell::new(transport);

        let temp_clone = Arc::clone(&temperature);
        let alive = Arc::new(atomic::AtomicBool::new(true));
        let alive_clone = Arc::clone(&alive);
        let handle = thread::spawn(move || {
            while alive_clone.load(atomic::Ordering::Relaxed) {
                let mut tt = t.borrow_mut();
                let temperature = match tt.communicate("") {
                    s => s.parse().unwrap_or_else(|e| {
                        eprintln!("Error parsing number: {}", e);
                        0.0f64
                    }),
                };
                if temperature != 0.0f64 {
                    let mut num = temp_clone.lock().unwrap();
                    *num = temperature;
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
        Self {
            temperature,
            handle,
            alive
        }
    }

    fn get_value(&self) -> f64 {
        *self.temperature.lock().unwrap()
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn get_state(&self) -> String {
        "ON".to_string()
    }

    fn on(&mut self) {}

    fn off(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;
    
    #[test]
    fn test_thermometer_get_value() {
        let thermometer = Thermometer::new(Box::new(MockTransport::new("25.0".to_string())));
        thread::sleep(Duration::from_secs(1));
        assert_eq!(thermometer.get_value(), 25.0);
    }
}
