use crate::smart_device::SmartDevice;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct Room {
    devises: HashMap<String, SmartDevice>,
}

impl Index<usize> for Room {
    type Output = SmartDevice;

    fn index(&self, name: &str) -> &Self::Output {
        &self.devises.get(name).expect(format!("Device name {} not found", name).as_str())
    }
}

impl IndexMut<usize> for Room {
    fn index_mut(&mut self, name: &str) -> &mut Self::Output {
        &mut self.devises.get(name).expect(format!("Device name {} not found", name).as_str())
    }
}

impl Room {
    pub fn new(devises: HashMap<String, SmartDevice>) -> Self {
        Room { devises }
    }

    pub fn print_status(&self) {
        for i in 0..self.devises.len() {
            self.devises[i].print_status();
        }
    }
}
