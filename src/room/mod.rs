use std::error::Error;
use std::fmt;
use crate::smart_device::SmartDevice;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[macro_export]
macro_rules! room {
    ( $( $key:tt : $device:expr ),* $(,)? ) => {{
        let mut room = Room::new();
        $(
            room.add_device($key, $device);
        )*
        room
    }};
}

#[derive(Debug)]
pub struct Room {
    devises: HashMap<String, SmartDevice>,
}

impl Index<&str> for Room {
    type Output = SmartDevice;

    fn index(&self, name: &str) -> &Self::Output {
        &self.devises.get(name).expect(format!("Device name {} not found", name).as_str())
    }
}

impl IndexMut<&str> for Room {
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        self.devises.get_mut(name).expect(&format!("Device name {} not found", name))
    }
}

impl Room {
    pub fn new() -> Self {
        Room { devises: Default::default() }
    }

    pub fn add_device(&mut self, name: &str, device: SmartDevice) {
        self.devises.insert(name.to_string(), device);
    }

    pub fn del_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.devises.remove(&name.to_string())
    }

    pub fn get_device(&self, name: &str) -> Result<&SmartDevice, Box<dyn Error>> {
        let device = self.devises.get(name).ok_or_else(|| {
            Box::new(DeviceError {
                message: name.to_string(),
            })
        })?;

        Ok(device)
    }

    pub fn print_status(&self) {
        println!("{:14}{:14}{:>6}{:>8}", "Name", "Type", "Status", "Value");
        for (name, room) in &self.devises {
            println!("{:14}{:?}", name, self.devises[name]);
        }
    }
}

#[derive(Debug)]
struct DeviceError {
    message: String
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Device {} not found", self.message)
    }
}

impl Error for DeviceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_device() {
        let mut room = Room::new();
        room.add_device("socket", SmartDevice::new_power_socket(100.0));
        room.del_device("socket");
        assert_eq!(room.devises.len(), 0);
    }

    #[test]
    fn test_del_device_not_exists() {
        let mut room = Room::new();
        room.add_device("socket1", SmartDevice::new_power_socket(100.0));
        let result = room.del_device("socket2");
        assert!(result.is_none());
        assert!(room.devises.contains_key("socket1"));
    }

    #[test]
    fn test_get_device_missing() {
        let room = Room::new();

        match room.get_device("socket") {
            Err(e) => {
                assert_eq!(e.to_string(), "Device socket not found");
            }
            _ => panic!("An error was expected, but it did not occur"),
        }
    }
}
