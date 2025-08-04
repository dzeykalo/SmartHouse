use crate::smart_device::SmartDevice;
use std::ops::{Index, IndexMut};

pub struct Room {
    devises: Vec<SmartDevice>,
}

impl Index<usize> for Room {
    type Output = SmartDevice;

    fn index(&self, index: usize) -> &Self::Output {
        &self.devises[index]
    }
}

impl IndexMut<usize> for Room {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.devises[index]
    }
}

impl Room {
    pub fn new(devises: Vec<SmartDevice>) -> Self {
        Room { devises }
    }

    pub fn print_status(&self) {
        for i in 0..self.devises.len() {
            self.devises[i].print_status();
        }
    }
}
