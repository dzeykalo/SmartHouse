use std::cell::RefCell;
use crate::device::Device;
use crate::transport::Transport;

pub struct PowerSocket {
    power: f64,
    transport: RefCell<Box<dyn Transport + Send>>,
}

impl Device for PowerSocket {
    fn new(transport: Box<dyn Transport + Send>) -> Self {
        Self {
            power: 0.00,
            transport: RefCell::new(transport),
        }
    }

    fn get_value(&self) -> f64 {
        match self.get_state().as_str() {
            "ON" => self.power,
            _ => 0.0,
        }
    }

    fn get_name(&self) -> String {
        String::from("PowerSocket")
    }

    fn get_state(&self) -> String {
        let mut transport = self.transport.borrow_mut();
        transport.communicate("state").to_ascii_uppercase()
    }

    fn on(&mut self) {
        self.transport.borrow_mut().communicate("on");
    }
    fn off(&mut self) {
        self.transport.borrow_mut().communicate("off");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;
    
    #[test]
    fn test_power_socket_initial_state() {
        let socket = PowerSocket::new(Box::new(MockTransport::new("OFF".to_string())));
        assert_eq!(socket.get_name(), "PowerSocket");
        assert_eq!(socket.get_value(), 0.0);
        assert_eq!(socket.get_state(), "OFF");
    }

    #[test]
    fn test_power_socket_turn_on_off() {
        let mut socket = PowerSocket::new(Box::new(MockTransport::new("ON".to_string())));
        socket.on();
        assert_eq!(socket.get_state(), "ON");
        assert_eq!(socket.get_value(), 0.0);

        socket.off();
        assert_eq!(socket.get_state(), "OFF");
        assert_eq!(socket.get_value(), 0.0);
    }
}
