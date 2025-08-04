use crate::device::Device;
use crate::power_socket::PowerSocket;
use crate::thermometer::Thermometer;
pub struct SmartDevice {
    device: Box<dyn Device>,
}

impl SmartDevice {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self { device }
    }

    pub fn new_thermometer(temp: f64) -> Self {
        Self::new(Box::new(Thermometer::new(temp)))
    }

    pub fn new_power_socket(temp: f64) -> Self {
        Self::new(Box::new(PowerSocket::new(temp)))
    }

    pub fn turn_on(&mut self) {
        self.device.on();
    }

    pub fn turn_off(&mut self) {
        self.device.off();
    }

    pub fn print_status(&self) {
        println!(
            "{:12} is {:>3}, current value: {:>3}",
            self.device.get_name(),
            if self.device.is_on() { "ON" } else { "OFF" },
            self.device.get_value()
        )
    }
}
