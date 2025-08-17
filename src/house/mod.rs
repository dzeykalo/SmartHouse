use std::collections::HashMap;
use crate::room::Room;
use std::ops::{Index, IndexMut};

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
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        House { rooms }
    }

    pub fn print_status(&self) {
        for (name, room) in &self.rooms {
            println!("Room {}:", name);
            self.rooms[name].print_status();
            println!();
        }
    }
}
