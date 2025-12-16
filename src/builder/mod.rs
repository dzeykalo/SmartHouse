use crate::house::House;
use crate::room::Room;
use crate::smart_device::SmartDevice;

#[derive(Default)]
pub struct HouseBuilder {
    pub house: House,
}

pub struct RoomBuilder<'a> {
    pub house: House,
    pub room_name: &'a str,
}

impl HouseBuilder {
    pub fn new() -> Self {
        HouseBuilder::default()
    }

    pub fn add_room(self, name: &str) -> RoomBuilder<'_> {
        RoomBuilder {
            house: self.house,
            room_name: name,
        }
    }

    pub fn build(self) -> House {
        self.house
    }
}

impl RoomBuilder<'_> {
    pub fn add_device(mut self, name: &str, device: SmartDevice) -> Self {
        let room = self.house.get_mut_room(self.room_name);
        if room.is_none() {
            self.house
                .add_room(self.room_name, Option::from(Room::new()));
        }
        self.house
            .get_mut_room(self.room_name)
            .unwrap()
            .add_device(name, device);
        self
    }

    pub fn build(self) -> HouseBuilder {
        HouseBuilder { house: self.house }
    }
}
