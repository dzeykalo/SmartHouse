use std::collections::HashMap;
use crate::room::Room;
use crate::room;
use std::ops::{Index, IndexMut};

#[macro_export]
macro_rules! house {
    ( $( $key:tt : $room:expr ),* $(,)? ) => {{
        let mut house = House::new();
        $(
            house.add_room($key, $room);
        )*
        house
    }};
}

#[derive(Debug)]
pub struct House {
    rooms: HashMap<String, Room>,
}

impl Index<&str> for House {
    type Output = Room;

    fn index(&self, name: &str) -> &Self::Output {
        &self.rooms.get(name).expect(format!("Room name {} not found", name).as_str())
    }
}

impl IndexMut<&str> for House {
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        self.rooms.get_mut(name).expect(&format!("Room name {} not found", name))
    }
}

impl House {
    pub fn new() -> Self {
        House { rooms: Default::default() }
    }

    pub fn add_room(&mut self, name: &str, room: Option<Room>) {
        let room = room.unwrap_or_else(Room::new);
        self.rooms.insert(name.to_string(), room);
    }

    pub fn del_room(&mut self, name: &str) {
        self.rooms.remove(&name.to_string());
    }

    pub fn print_status(&self) {
        for (name, room) in &self.rooms {
            println!("Room: {}", name);
            self.rooms[name].print_status();
            println!();
        }
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
}