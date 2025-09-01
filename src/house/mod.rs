use crate::reportable::Reportable;
use crate::room::Room;
use crate::smart_device::SmartDevice;
use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! house {
    ( $( $key:tt : $room:expr ),* $(,)? ) => {{
        let mut house = House::new();
        $(
            house.add_room($key, Option::from($room));
        )*
        house
    }};
}

#[derive(Debug)]
pub struct House {
    rooms: HashMap<String, Room>,
}

impl Default for House {
    fn default() -> Self {
        Self::new()
    }
}

impl House {
    pub fn new() -> Self {
        House {
            rooms: Default::default(),
        }
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn get_mut_room(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn add_room(&mut self, name: &str, room: Option<Room>) {
        let room = room.unwrap_or_default();
        self.rooms.insert(name.to_string(), room);
    }

    pub fn del_room(&mut self, name: &str) {
        self.rooms.remove(name);
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, Box<dyn Error>> {
        let room = self
            .rooms
            .get(room_name)
            .ok_or_else(|| format!("Room not found: {}", room_name))?;

        let device = room
            .get_device(device_name)
            .ok_or_else(|| format!("Device not found: {}", device_name))?;

        Ok(device)
    }
}

impl Reportable for House {
    fn generate_report(&self) -> String {
        self.rooms
            .iter()
            .map(|(name, device)| format!("Room: {}\n{}\n", name, device.generate_report()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_delete_room() {
        let mut house = House::new();
        assert!(!house.rooms.contains_key("Test room"));

        house.add_room("Test room", None);
        assert!(house.rooms.contains_key("Test room"));

        house.del_room("Test room");
        assert!(!house.rooms.contains_key("Test room"));
    }

    #[test]
    fn test_get_device_room_not_found() {
        let house = House::new();

        let result = house.get_device("kitchen", "thermometer");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Room not found: kitchen");
    }
}
