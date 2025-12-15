use crate::report::{Report, Reportable};
use crate::smart_device::SmartDevice;
use crate::subscriber::Subscriber;
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

pub struct Room {
    devises: HashMap<String, SmartDevice>,
    subscribers: Vec<Box<dyn Subscriber>>,
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
            subscribers: Vec::new(),
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
        
        for subscriber in self.subscribers.iter_mut() {
            subscriber.on_event();
        }
    }

    pub fn del_device(&mut self, name: &str) -> Option<SmartDevice> {
        self.devises.remove(name)
    }

    pub fn get_devices_names(&self) -> Vec<String> {
        self.devises.keys().cloned().collect()
    }
    
    pub fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }
}

impl Reportable for Room {
    fn generate_report(&self) -> String {
        format!(
            "{:14}{:14}{:14}{:>6}\n{}",
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

impl Report for Room {
    fn report(&self) -> String {
        format!("Rooms contains {} devices witch names: {:?}", self.devises.len(), self.get_devices_names())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_del_device() {
        let mut room = Room::new();
        room.add_device(
            "socket",
            SmartDevice::new_power_socket("localhost", 8080, 60.0f64),
        );
        room.del_device("socket");
        assert_eq!(room.devises.len(), 0);
    }

    #[test]
    fn test_del_device_not_exists() {
        let mut room = Room::new();
        room.add_device(
            "socket1",
            SmartDevice::new_power_socket("localhost", 8080, 60.0f64),
        );
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
