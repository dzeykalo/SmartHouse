use std::collections::HashMap;
use smart_house_lib::house::House;
// use smart_house_lib::room;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;


fn main() {
    let mut house = House::new();

    // let room = room! (
    //     name: "Kitchen",
    //     devices: [
    //         thermometer: SmartDevice::new_thermometer(20.0),
    //         socket: SmartDevice::new_power_socket(100.0)
    //     ]
    // );

    house.add_room("Living room");
    house["Living room"].add_device(
        "thermometer", SmartDevice::new_thermometer(23.0)
    );
    house["Living room"].add_device(
        "socket1", SmartDevice::new_power_socket(100.0)
    );
    house["Living room"].add_device(
        "socket2", SmartDevice::new_power_socket(60.0)
    );

    house.add_room("Dining room");
    house["Dining room"].add_device(
        "thermometer", SmartDevice::new_thermometer(25.0)
    );
    house["Dining room"].add_device(
        "socket1", SmartDevice::new_power_socket(40.0)
    );
    house["Dining room"].add_device(
        "socket2", SmartDevice::new_power_socket(60.0)
    );
    house["Dining room"]["socket1"].turn_on();

    house.add_room("Test room");
    house.del_room("Test room");

    house.print_status();

}
