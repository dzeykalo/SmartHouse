use crate::smart_device::SmartDevice;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
    pub fn new(devises: HashMap<String, SmartDevice>) -> Self {
        Room { devises }
    }

    pub fn print_status(&self) {
        for (name, room) in &self.devises {
            println!("Devise {}:", name);
            self.devises[name].print_status();
            println!();
        }
    }
}
