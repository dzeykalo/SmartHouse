use smart_house_lib::house;
use smart_house_lib::house::House;
use smart_house_lib::room;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;
mod cli;
use cli::{run_cli_loop, get_port_then_increment};


fn main() {
    let mut therm_port = 5000;
    let mut socket_port = 6000;

    let house = house!(
        "living room": room!(
            "thermo_5000": SmartDevice::new_thermometer("127.0.0.1", get_port_then_increment(&mut therm_port), 0.0f64),
            "socket_6000": SmartDevice::new_power_socket("127.0.0.1", get_port_then_increment(&mut socket_port), 60.0f64),
        ),
        "kitchen": room!(
            "thermo_5001": SmartDevice::new_thermometer("127.0.0.1", get_port_then_increment(&mut therm_port), 0.0f64),
            "socket_6001": SmartDevice::new_power_socket("127.0.0.1", get_port_then_increment(&mut socket_port), 40.0f64),
        )
    );

    println!("Smart House CLI started!");
    run_cli_loop(house, therm_port, socket_port);
}
