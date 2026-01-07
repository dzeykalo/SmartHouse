use smart_house_lib::house::House;
use smart_house_lib::report::Reportable;
use smart_house_lib::smart_device::SmartDevice;
use std::io;
use std::io::Write;

pub fn run_cli_loop(mut house: House) {
    loop {
        println!("\nCommands:");
        println!("  1 - Add device");
        println!("  2 - Remove device");
        println!("  3 - Tern on device");
        println!("  4 - Tern off device");
        println!("  5 - Show report");
        println!("  0 - Exit");
        print!("\nEnter command (0-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        let command = input.trim().parse::<u8>().unwrap_or(255);
        input.clear();
        match command {
            1 => {
                // Add device
                let room_name = get_name(&house.get_rooms_names());
                if room_name.is_empty() {
                    println!("Invalid room number.");
                    continue;
                }

                println!("Select device type:");
                println!("  1 - Thermometer");
                println!("  2 - Power Socket");
                print!("Choose (1-2): ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let dev_type = input.trim().parse::<u8>().unwrap_or(0);

                let (mut device_name, device) = match dev_type {
                    1 => {
                        let device_name = "Thermometer".to_string();
                        (device_name, SmartDevice::thermometer(21.5f64))
                    }
                    2 => {
                        let device_name = "PowerSocket".to_string();
                        (device_name, SmartDevice::power_socket(50.0f64))
                    }
                    _ => {
                        println!("Invalid device type.");
                        continue;
                    }
                };

                if let Some(room) = house.get_mut_room(&room_name) {
                    device_name = format!("{}_{}", device_name, room.get_devices_names().len() + 1);
                    room.add_device(&device_name, device);
                    println!("Device '{}' added to room '{}'.", device_name, room_name);
                }
            }
            2 => {
                // Remove device
                let room_name = get_name(&house.get_rooms_names());
                if room_name.is_empty() {
                    println!("Invalid room number.");
                    continue;
                }

                if let Some(room) = house.get_mut_room(&room_name) {
                    let device_name = get_name(&room.get_devices_names());
                    if device_name.is_empty() {
                        println!("Invalid device number.");
                        continue;
                    }
                    room.del_device(&device_name);
                    println!(
                        "Device '{}' removed from room '{}'.",
                        device_name, room_name
                    );
                }
            }
            3 => {
                // Tern on
                let room_name = get_name(&house.get_rooms_names());
                if room_name.is_empty() {
                    println!("Invalid room number.");
                    continue;
                }
                if let Some(room) = house.get_mut_room(&room_name) {
                    let device_name = get_name(&room.get_devices_names());
                    if device_name.is_empty() {
                        println!("Invalid device number.");
                        continue;
                    }
                    if let Some(device) = room.get_mut_device(&device_name) {
                        device.turn_on();
                        println!("Device '{}' turned on.", device_name);
                    }
                }
            }
            4 => {
                // Tern off
                let room_name = get_name(&house.get_rooms_names());
                if room_name.is_empty() {
                    println!("Invalid room number.");
                    continue;
                }
                if let Some(room) = house.get_mut_room(&room_name) {
                    let device_name = get_name(&room.get_devices_names());
                    if device_name.is_empty() {
                        println!("Invalid device number.");
                        continue;
                    }
                    if let Some(device) = room.get_mut_device(&device_name) {
                        device.turn_off();
                        println!("Device '{}' turned on.", device_name);
                    }
                }
            }
            5 => {
                print_report(&house);
            }
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command. Use 0-5.");
            }
        }
    }
}

fn print_report<T: Reportable>(x: &T) {
    println!("{}", x.generate_report());
}

fn get_name(names_list: &[String]) -> String {
    println!("Available:");
    for (i, name) in names_list.iter().enumerate() {
        println!("  {}: {}", i + 1, name);
    }

    print!("Select number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let idx = input.trim().parse::<usize>().unwrap_or(0);
    if idx == 0 || idx > names_list.len() {
        return String::new();
    }
    names_list[idx - 1].clone()
}
