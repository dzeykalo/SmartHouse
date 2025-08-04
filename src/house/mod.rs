use crate::room::Room;
use std::ops::{Index, IndexMut};

pub struct House {
    rooms: Vec<Room>,
}

impl Index<usize> for House {
    type Output = Room;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rooms[index]
    }
}

impl IndexMut<usize> for House {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rooms[index]
    }
}

impl House {
    pub fn new(rooms: Vec<Room>) -> Self {
        House { rooms }
    }

    pub fn print_status(&self) {
        for i in 0..self.rooms.len() {
            println!("Room {}:", i);
            self.rooms[i].print_status();
            println!();
        }
    }
}
