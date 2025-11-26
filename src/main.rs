use smart_house_lib::house;
use smart_house_lib::house::House;
use smart_house_lib::reportable::Reportable;
use smart_house_lib::room;
use smart_house_lib::room::Room;
use smart_house_lib::smart_device::SmartDevice;

fn print_report<T: Reportable>(x: &T) {
    println!("{}\n", x.generate_report());
}

fn main() {
    let house = house!(
        "living room": room!(
            "thermometer": SmartDevice::new_thermometer("127.0.0.1", 5000),
            "socket1": SmartDevice::new_power_socket("127.0.0.1", 6000),
        ),
        "kitchen": room!(
            "thermometer": SmartDevice::new_thermometer("127.0.0.1", 5001),
            "socket1": SmartDevice::new_power_socket("127.0.0.1", 6001),
        )
    );
    loop {
        println!("\nPlease press enter to show report...");
        if std::io::stdin().read_line(&mut String::new()).is_ok() {
            print_report(&house);
        }
    }

    // house.add_room("hall", None);
    // if let Some(room) = house.get_mut_room("hall") {
    //     room.add_device("thermometer", SmartDevice::new_thermometer());
    // }
    // print_report(house.get_room("hall").unwrap());
    // if let Some(room) = house.get_mut_room("hall") {
    //     room.del_device("thermometer");
    // }
    // house.del_room("hall");
    // print_report(&house);
    // 
    // if house.get_room("hall").is_none() {
    //     println!("hall room doesn't exist");
    // }
}
