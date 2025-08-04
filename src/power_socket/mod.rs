use crate::device::Device;

pub enum PowerSocketState {
    OFF,
    ON,
}

pub struct PowerSocket {
    power: f64,
    state: PowerSocketState,
}

impl Device for PowerSocket {
    fn new(w: f64) -> Self {
        Self {
            power: w,
            state: PowerSocketState::OFF,
        }
    }

    fn is_on(&self) -> bool {
        match self.state {
            PowerSocketState::ON => true,
            PowerSocketState::OFF => false,
        }
    }

    fn get_value(&self) -> f64 {
        match self.state {
            PowerSocketState::ON => self.power,
            _ => 0.0,
        }
    }

    fn get_name(&self) -> String {
        String::from("PowerSocket")
    }

    fn on(&mut self) {
        self.state = PowerSocketState::ON
    }
    fn off(&mut self) {
        self.state = PowerSocketState::OFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_socket_initial_state() {
        let socket = PowerSocket::new(60.0);
        assert_eq!(socket.get_name(), "PowerSocket");
        assert_eq!(socket.get_value(), 0.0);
        assert_eq!(socket.is_on(), false);
    }

    #[test]
    fn test_power_socket_turn_on_off() {
        let mut socket = PowerSocket::new(0.0);
        socket.on();
        assert_eq!(socket.is_on(), true);
        assert_eq!(socket.get_value(), 0.0);

        socket.off();
        assert_eq!(socket.is_on(), false);
        assert_eq!(socket.get_value(), 0.0);
    }
}
