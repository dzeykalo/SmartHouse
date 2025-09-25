use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::device::Device;
use crate::transport::Transport;

// #[derive(Debug)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    handle: thread::JoinHandle<()>
}

impl Thermometer {
    
}

impl Device for Thermometer {
    fn new(transport: Box<dyn Transport + Send>, t: f64) -> Self {
        let temperature = Arc::new(Mutex::new(0.0));
        let t = RefCell::new(transport);

        let temp_clone = Arc::clone(&temperature);
        let handle = thread::spawn(move || {
            let mut tt = t.borrow_mut();
            let temperature = match tt.receive().as_str() {
                "" => 0.0f64,
                s => s.parse().unwrap_or_else(|e| {
                    eprintln!("Error parsing number: {}", e);
                    0.0f64
                }),
            };
            let mut num = temp_clone.lock().unwrap();
            *num = temperature;
        });
        Self {
            temperature,
            handle,
        }
    }

    fn is_on(&self) -> bool {
        true
    }

    fn get_value(&self) -> f64 {
        *self.temperature.lock().unwrap()
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn on(&mut self) {}

    fn off(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer::new(Box::new(MockTransport::new("".to_string())), 23.0);
        assert_eq!(thermometer.get_name(), "Thermometer");
        assert_eq!(thermometer.get_value(), 23.0);
        assert_eq!(thermometer.is_on(), true);
    }

    #[test]
    fn test_thermometer_turn_on_off() {
        let mut thermometer = Thermometer::new(Box::new(MockTransport::new("".to_string())), 25.0);
        thermometer.on();
        assert_eq!(thermometer.is_on(), true);

        thermometer.off();
        assert_eq!(thermometer.is_on(), true);
    }
}
