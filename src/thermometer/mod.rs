use std::cell::RefCell;
use std::sync::{atomic, mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::device::Device;
use crate::transport::Transport;

#[derive(Debug)]
enum ThermometerState {
    ON,
    OFF
}

#[derive(Debug)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    // handle: Option<thread::JoinHandle<()>>,
    alive: Arc<atomic::AtomicBool>,
    state: ThermometerState
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        self.alive.store(false, atomic::Ordering::Relaxed);
        // if let Some(handle) = self.handle.take() {
        //     let _ = handle.join();
        // }
    }
}

impl Device for Thermometer {
    fn new(transport: Box<dyn Transport + Send>) -> Self {
        let temperature = Arc::new(Mutex::new(0.0));
        let alive = Arc::new(atomic::AtomicBool::new(true));

        let temp_clone = Arc::clone(&temperature);
        let alive_clone = Arc::clone(&alive);
        let handle = thread::spawn(move || {
            let t = RefCell::new(transport);
            let mut tt = t.borrow_mut();
            
            while alive_clone.load(atomic::Ordering::Relaxed) {
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
            // handle: Some(handle),
            alive,
            state: ThermometerState::ON
        }
    }

    fn get_value(&self) -> f64 {
        match self.state {
            ThermometerState::ON => 0.0f64,
            ThermometerState::OFF => *self.temperature.lock().unwrap()
        }
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn get_state(&self) -> String {
        match self.state {
            ThermometerState::ON => String::from("ON"),
            ThermometerState::OFF => String::from("OFF")
        }
    }

    fn on(&mut self) {
        self.state = ThermometerState::ON;
    }

    fn off(&mut self) {
        self.state = ThermometerState::OFF;
    }
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
