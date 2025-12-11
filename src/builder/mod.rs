use smart_house_lib::house::House;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;

#[derive(Debug, Default)]
pub struct HouseBuilder {
    pub house: House,
}

impl HouseBuilder {
    pub fn add_room(mut self, name: &str, room: Option<Room>) -> Self {
        self.house.add_room(name,room);
        self
    }
    
    pub fn add_device(mut self, name: &str, device: SmartDevice) -> Self {
        self.house["Living Room"].add_device(name, device);
        self
    }
}

