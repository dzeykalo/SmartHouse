use std::cell::RefCell;
use std::io::{self, prelude::*, BufReader};
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

    fn is_on(&self) -> bool {
        let mut transport = self.transport.borrow_mut();
        transport.send("state");
        match transport.receive().as_str() {
            "on" => true,
            _ => false,
        }
    }

    fn get_value(&self) -> f64 {
        match self.is_on() {
            true => self.power,
            _ => 0.0,
        }
    }

    fn get_name(&self) -> String {
        String::from("PowerSocket")
    }

    fn on(&mut self) {
        self.transport.borrow_mut().send("on");
    }
    fn off(&mut self) {
        self.transport.borrow_mut().send("off");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;
    
    #[test]
    fn test_power_socket_initial_state() {
        let mut socket = PowerSocket::new(Box::new(MockTransport::new("".to_string())));
        assert_eq!(socket.get_name(), "PowerSocket");
        assert_eq!(socket.get_value(), 0.0);
        assert_eq!(socket.is_on(), false);
    }

    #[test]
    fn test_power_socket_turn_on_off() {
        let mut socket = PowerSocket::new(Box::new(MockTransport::new("".to_string())));
        socket.on();
        assert_eq!(socket.is_on(), true);
        assert_eq!(socket.get_value(), 0.0);

        socket.off();
        assert_eq!(socket.is_on(), false);
        assert_eq!(socket.get_value(), 0.0);
    }
}
