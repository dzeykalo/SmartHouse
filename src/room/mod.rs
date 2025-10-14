use crate::reportable::Reportable;
use crate::smart_device::SmartDevice;
use std::collections::HashMap;

#[macro_export]
macro_rules! room {
    ( $( $key:tt : $device:expr ),* $(,)? ) => {{
        let mut room = Room::new();
        $(
            room.add_device($key, $device);
        )*
        Option::Some(room)
    }};
}

#[derive(Debug)]
pub struct Room {
    devises: HashMap<String, SmartDevice>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Room {
    pub fn new() -> Self {
        Room {
            devises: Default::default(),
        }
    }

    pub fn get_device(&self, name: &str) -> Option<&SmartDevice> {
        self.devises.get(name)
    }

    pub fn get_mut_device(&mut self, name: &str) -> Option<&mut SmartDevice> {
        self.devises.get_mut(name)
    }

    pub fn add_device(&mut self, name: &str, device: SmartDevice) {
        self.devises.insert(name.to_string(), device);
    }

    pub fn del_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.devises.remove(name)
    }
}

impl Reportable for Room {
    fn generate_report(&self) -> String {
        format!(
            "{:14}{:14}{:>6}{:>8}\n{}",
            "Name",
            "Type",
            "Status",
            "Value",
            self.devises
                .iter()
                .map(|(name, device)| { format!("{:14}{}", name, device.generate_report()) })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_device() {
        let mut room = Room::new();
        room.add_device("socket", SmartDevice::new_power_socket());
        room.del_device("socket");
        assert_eq!(room.devises.len(), 0);
    }

    #[test]
    fn test_del_device_not_exists() {
        let mut room = Room::new();
        room.add_device("socket1", SmartDevice::new_power_socket());
        let result = room.del_device("socket2");
        assert!(result.is_none());
        assert!(room.devises.contains_key("socket1"));
    }

    #[test]
    fn test_get_device_missing() {
        let room = Room::new();

        let result = room.get_device("socket");
        assert!(result.is_none());
    }
}
