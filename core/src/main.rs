use smart_house_lib::builder::HouseBuilder;
use smart_house_lib::smart_device::SmartDevice;
mod cli;
use cli::run_cli_loop;

fn main() {
    let house = HouseBuilder::new()
        .add_room("First room")
        .add_device("PowerSocket_1", SmartDevice::power_socket(40.0))
        .add_device("PowerSocket_2", SmartDevice::power_socket(60.0))
        .add_device("Thermometer_1", SmartDevice::thermometer(23.0))
        .build()
        .add_room("Second room")
        .add_device("PowerSocket_1", SmartDevice::power_socket(40.0))
        .add_device("Thermometer_1", SmartDevice::thermometer(23.2))
        .build()
        .build();

    println!("Smart House CLI started!");
    run_cli_loop(house);
}
