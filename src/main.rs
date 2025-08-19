use std::collections::HashMap;
use smart_house_lib::house::House;
use smart_house_lib::house;
use smart_house_lib::room;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;


fn main() {
    let mut house = house!(
        "living room": room!(
            "thermometer": SmartDevice::new_thermometer(23.0),
            "socket1": SmartDevice::new_power_socket(100.0),
            "socket2": SmartDevice::new_power_socket(60.0)
        ),
        "kitchen": room!(
            "thermometer": SmartDevice::new_thermometer(25.0),
            "socket1": SmartDevice::new_power_socket(40.0),
            "socket2": SmartDevice::new_power_socket(60.0)
        )
    );

    house["kitchen"]["socket1"].turn_on();
    house.print_status();


    // house.add_room("Test room");
    // house.del_room("Test room");
}
