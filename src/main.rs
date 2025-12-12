use smart_house_lib::smart_device::SmartDevice;
use smart_house_lib::builder::HouseBuilder;
mod cli;
use cli::run_cli_loop;

fn main() {
    let mut therm_port = 5000;
    let mut socket_port = 6000;

    let house = HouseBuilder::new()
        .add_room("First room")
            .add_device("Socket_1", SmartDevice::new_local_power_socket(&mut socket_port))
            .add_device("Socket_2", SmartDevice::new_local_power_socket(&mut socket_port))
            .add_device("Thermo_1", SmartDevice::new_local_thermometer(&mut therm_port))
            .build()
        .add_room("Second room")
            .add_device("Socket_3", SmartDevice::new_local_power_socket(&mut socket_port))
            .add_device("Thermo_2", SmartDevice::new_local_thermometer(&mut therm_port))
            .build()
        .build();

    println!("Smart House CLI started!");
    run_cli_loop(house, therm_port, socket_port);
}
