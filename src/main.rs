use smart_house_lib::smart_device::SmartDevice;
use smart_house_lib::builder::HouseBuilder;
use smart_house_lib::report::Reporter;
use smart_house_lib::subscriber::LoggingSubscriber;
mod cli;
use cli::run_cli_loop;
use smart_house_lib::house::House;
use smart_house_lib::power_socket::PowerSocket;
use smart_house_lib::room::Room;
use smart_house_lib::thermometer::Thermometer;

fn main() {
    let mut room = Room::default();
    room.subscribe(Box::new(LoggingSubscriber::default()));
    room.add_device("Socket_1", SmartDevice::new_local_power_socket(&mut 5000));
    
    let house = House::default();
    let room = Room::default();
    let socket1 = PowerSocket::default();
    let socket2 = PowerSocket::default();
    let thermo1 = Thermometer::default();
    let thermo2 = Thermometer::default();
    let _reporter = Reporter::new()
        .add(&house)
        .add(&room)
        .add(&socket1)
        .add(&socket2)
        .add(&thermo1)
        .add(&thermo2)
        .report();
    
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
