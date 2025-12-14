use crate::device::Device;
use crate::power_socket::PowerSocket;
use crate::report::Reportable;
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

    pub fn new_thermometer(ip: &str, port: u16, temperature: f64) -> Self {
        let transport = UdpTransport::new(ip, port);
        Self::new(Box::new(Thermometer::new(Box::new(transport), temperature)))
    }

    pub fn new_local_thermometer(port: &mut u16) -> Self {
        Self::new_thermometer("127.0.0.1", Self::get_port_then_increment(port), 23.0f64)
    }

    pub fn new_power_socket(ip: &str, port: u16, wattage: f64) -> Self {
        let transport = TcpTransport::new(ip, port);
        Self::new(Box::new(PowerSocket::new(Box::new(transport), wattage)))
    }

    pub fn new_local_power_socket(port: &mut u16) -> Self {
        Self::new_power_socket("127.0.0.1", Self::get_port_then_increment(port), 60.0f64)
    }

    pub fn turn_on(&mut self) {
        self.device.on();
    }

    pub fn turn_off(&mut self) {
        self.device.off();
    }

    pub fn get_port_then_increment(base: &mut u16) -> u16 {
        if *base == 65535 {
            panic!("No more ports available");
        }
        let port = *base;
        *base += 1;
        port
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
        let thermometer = Thermometer::new(Box::new(MockTransport::new("".to_string())), 23.0f64);
        let smart_device = SmartDevice::from(thermometer);
        assert_eq!(smart_device.device.get_value(), 23.0);
    }

    #[test]
    fn test_smart_device_from_power_socket() {
        let power_socket = PowerSocket::new(Box::new(MockTransport::new("".to_string())), 60.0f64);
        let smart_device = SmartDevice::from(power_socket);
        assert_eq!(smart_device.device.get_value(), 0.0);
    }
}
