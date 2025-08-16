use smart_house_lib::house::House;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;
use serde::Deserialize;

fn main() {
    let room = Room::new(hashmap! {
            "therm".to_string() => SmartDevice::new_thermometer(24.0),
            "power1".to_string() => SmartDevice::new_power_socket(40.0),
            "power2".to_string() => SmartDevice::new_power_socket(60.0),
        });

    // let mut house = House::new(vec![
    //     Room::new(hashmap! {
    //         "thermo".to_string() => SmartDevice::new_thermometer(24.0),
    //         "power1".to_string() => SmartDevice::new_power_socket(40.0),
    //         "power2".to_string() => SmartDevice::new_power_socket(60.0),
    //     }),
    //     Room::new(hashmap! {
    //         SmartDevice::new_thermometer(23.0),
    //         SmartDevice::new_power_socket(100.0),
    //     }),
    // ]);
    // house[1][0].turn_off();
    // house[1][1].turn_on();
    // house.print_status();
}
