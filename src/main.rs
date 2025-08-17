use std::collections::HashMap;
use smart_house_lib::house::House;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;


fn main() {
    let mut living_room_devises = HashMap::new();
    living_room_devises.insert("thermometer".to_string(), SmartDevice::new_thermometer(24.0));
    living_room_devises.insert("socket1".to_string(), SmartDevice::new_power_socket(40.0));
    living_room_devises.insert("socket2".to_string(), SmartDevice::new_power_socket(60.0));

    let mut dining_room_devises = HashMap::new();
    dining_room_devises.insert("thermometer".to_string(), SmartDevice::new_thermometer(23.0));
    dining_room_devises.insert("socket1".to_string(), SmartDevice::new_power_socket(100.0));
    
    let mut rooms = HashMap::new();
    rooms.insert("Living room".to_string(), Room::new(living_room_devises));
    rooms.insert("Dining room".to_string(), Room::new(dining_room_devises));

    let mut house = House::new(
        rooms
    );
    
    house["Living room"]["socket1"].turn_off();
    house["Living room"]["socket1"].turn_on();
    house.print_status();
}
