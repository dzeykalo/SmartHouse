use crate::device::Device;

pub struct PowerSocket {
    power: f64,
    state: String,
}

impl Default for PowerSocket {
    fn default() -> Self {
        Self {
            power: 0.0,
            state: "OFF".to_string(),
        }
    }
}

impl Device for PowerSocket {
    fn new(w: f64) -> Self {
        Self {
            power: w,
            state: "OFF".to_string(),
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
        self.state.clone()
    }

    fn on(&mut self) {
        self.state = "ON".to_string();
    }
    fn off(&mut self) {
        self.state = "OFF".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_socket_initial_state() {
        let socket = PowerSocket::new(60.0);
        assert_eq!(socket.get_name(), "PowerSocket".to_string());
        assert_eq!(socket.get_value(), 0.0);
        assert_eq!(socket.get_state(), "OFF".to_string());
    }

    #[test]
    fn test_power_socket_turn_on_off() {
        let mut socket = PowerSocket::new(60.0);
        socket.on();
        assert_eq!(socket.get_state(), "ON".to_string());
        assert_eq!(socket.get_value(), 60.0);

        socket.off();
        assert_eq!(socket.get_state(), "OFF".to_string());
        assert_eq!(socket.get_value(), 0.0);
    }
}
