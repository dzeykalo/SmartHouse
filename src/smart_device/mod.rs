use crate::device::Device;
use crate::power_socket::PowerSocket;
use crate::reportable::Reportable;
use crate::thermometer::Thermometer;
use crate::transport::{TcpTransport, UdpTransport};
use std::fmt::{self, Debug, Formatter};

pub struct SmartDevice {
    device: Box<dyn Device>,
}

impl SmartDevice {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self { device }
    }

    pub fn new_thermometer(ip: &str, port: u16) -> Self {
        let transport = UdpTransport::new(ip, port);
        Self::new(Box::new(Thermometer::new(Box::new(transport))))
    }

    pub fn new_power_socket(ip: &str, port: u16) -> Self {
        let transport = TcpTransport::new(ip, port);
        Self::new(Box::new(PowerSocket::new(Box::new(transport))))
    }

    pub fn turn_on(&mut self) {
        self.device.on();
    }

    pub fn turn_off(&mut self) {
        self.device.off();
    }
}

impl Debug for SmartDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:14}{:14}{:>6}",
            self.device.get_name(),
            self.device.get_state(),
            self.device.get_value()
        )
    }
}

impl From<Thermometer> for SmartDevice {
    fn from(device: Thermometer) -> Self {
        Self::new(Box::new(device))
    }
}

impl From<PowerSocket> for SmartDevice {
    fn from(device: PowerSocket) -> Self {
        Self::new(Box::new(device))
    }
}

impl Reportable for SmartDevice {
    fn generate_report(&self) -> String {
        format!(
            "{:14}{:14}{:>6}",
            self.device.get_name(),
            self.device.get_state(),
            self.device.get_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;
    
    #[test]
    fn test_smart_device_from_thermometer() {
        let thermometer = Thermometer::new(Box::new(MockTransport::new("".to_string())));
        let smart_device = SmartDevice::from(thermometer);
        assert_eq!(smart_device.device.get_value(), 23.0);
    }

    #[test]
    fn test_smart_device_from_power_socket() {
        let power_socket = PowerSocket::new(Box::new(MockTransport::new("".to_string())));
        let smart_device = SmartDevice::from(power_socket);
        assert_eq!(smart_device.device.get_value(), 0.0);
    }
}
