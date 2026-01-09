use crate::report::Reportable;
use libloading::{Library, Symbol};
use std::ffi::c_uint;
use std::fmt::{self, Debug, Formatter};
use std::os::raw::c_char;

#[cfg(target_os = "macos")]
const LIB_EXTENSION: &str = "dylib";
#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = "so";
#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";

fn lib_path() -> String {
    format!("./target/debug/libdevices.{}", LIB_EXTENSION)
}

#[repr(C)]
pub enum DeviceType {
    PowerSocket,
    Thermometer,
}

pub struct SmartDevice {
    device_id: c_uint,
    device_type: DeviceType,
    lib: Option<Library>,
}

unsafe extern "C" {
    fn new_device(device_type: DeviceType, value: f64) -> c_uint;
    fn device_on(id: c_uint) -> bool;
    fn device_off(id: c_uint) -> bool;
    fn device_get_value(id: c_uint) -> f64;
    fn device_get_name(id: c_uint, buffer: *mut c_char, len: usize) -> usize;
    fn device_state(id: c_uint, buffer: *mut c_char, len: usize) -> usize;
}

type NewDevice = unsafe extern "C" fn(device_type: DeviceType, value: f64) -> c_uint;
type DeviceOn = unsafe extern "C" fn(id: c_uint) -> bool;
type DeviceOff = unsafe extern "C" fn(id: c_uint) -> bool;
type DeviceGetValue = unsafe extern "C" fn(id: c_uint) -> f64;
type DeviceGetName = unsafe extern "C" fn(id: c_uint, buffer: *mut c_char, len: usize) -> usize;
type DeviceState = unsafe extern "C" fn(id: c_uint, buffer: *mut c_char, len: usize) -> usize;

impl SmartDevice {
    fn new(device_id: c_uint, device_type: DeviceType, lib: Option<Library>) -> Self {
        Self {
            device_id,
            device_type,
            lib,
        }
    }

    pub fn thermometer(temperature: f64) -> Self {
        unsafe {
            let lib = Library::new(lib_path()).unwrap();
            let new_device: Symbol<'_, NewDevice> = lib.get(b"new_device").unwrap();
            let id = new_device(DeviceType::Thermometer, temperature);
            Self::new(id, DeviceType::Thermometer, Some(lib))
        }
    }

    pub fn power_socket(wattage: f64) -> Self {
        let id = unsafe { new_device(DeviceType::PowerSocket, wattage) };
        Self::new(id, DeviceType::PowerSocket, None)
    }

    fn call_device_on(&self) -> bool {
        match self.device_type {
            DeviceType::Thermometer => unsafe {
                let lib = self.lib.as_ref().unwrap();
                let func: Symbol<'_, DeviceOn> = lib.get(b"device_on").unwrap();
                func(self.device_id)
            },
            DeviceType::PowerSocket => unsafe { device_on(self.device_id) },
        }
    }

    fn call_device_off(&self) -> bool {
        match self.device_type {
            DeviceType::Thermometer => unsafe {
                let lib = self.lib.as_ref().unwrap();
                let func: Symbol<'_, DeviceOff> = lib.get(b"device_off").unwrap();
                func(self.device_id)
            },
            DeviceType::PowerSocket => unsafe { device_off(self.device_id) },
        }
    }

    fn call_device_get_value(&self) -> f64 {
        match self.device_type {
            DeviceType::Thermometer => unsafe {
                let lib = self.lib.as_ref().unwrap();
                let func: Symbol<'_, DeviceGetValue> = lib.get(b"device_get_value").unwrap();
                func(self.device_id)
            },
            DeviceType::PowerSocket => unsafe { device_get_value(self.device_id) },
        }
    }

    fn call_device_get_name(&self) -> String {
        let mut buffer = [0i8; 32];
        let written = match self.device_type {
            DeviceType::Thermometer => unsafe {
                let lib = self.lib.as_ref().unwrap();
                let func: Symbol<'_, DeviceGetName> = lib.get(b"device_get_name").unwrap();
                func(self.device_id, buffer.as_mut_ptr(), buffer.len())
            },
            DeviceType::PowerSocket => unsafe {
                device_get_name(self.device_id, buffer.as_mut_ptr(), buffer.len())
            },
        };
        let slice = &buffer[..written.min(buffer.len() - 1)];
        let name_bytes: Vec<u8> = slice.iter().map(|&b| b as u8).collect();
        String::from_utf8_lossy(&name_bytes).into_owned()
    }

    fn call_device_state(&self) -> String {
        let mut buffer = [0i8; 32];
        let written = match self.device_type {
            DeviceType::Thermometer => unsafe {
                let lib = self.lib.as_ref().unwrap();
                let func: Symbol<'_, DeviceState> = lib.get(b"device_state").unwrap();
                func(self.device_id, buffer.as_mut_ptr(), buffer.len())
            },
            DeviceType::PowerSocket => unsafe {
                device_state(self.device_id, buffer.as_mut_ptr(), buffer.len())
            },
        };
        let slice = &buffer[..written.min(buffer.len() - 1)];
        let state_bytes: Vec<u8> = slice.iter().map(|&b| b as u8).collect();
        String::from_utf8_lossy(&state_bytes).into_owned()
    }

    pub fn turn_on(&mut self) {
        self.call_device_on();
    }

    pub fn turn_off(&mut self) {
        self.call_device_off();
    }

    pub fn get_value(&self) -> f64 {
        self.call_device_get_value()
    }

    fn get_name(&self) -> String {
        self.call_device_get_name()
    }

    fn get_state(&self) -> String {
        self.call_device_state()
    }
}

impl Debug for SmartDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:14}{:14}{:>6}",
            self.get_name(),
            self.get_state(),
            self.get_value()
        )
    }
}

impl Reportable for SmartDevice {
    fn generate_report(&self) -> String {
        format!(
            "{:14}{:14}{:>6}",
            self.get_name(),
            self.get_state(),
            self.get_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_socket_creation() {
        let device = SmartDevice::power_socket(120.0);
        assert_eq!(device.get_name(), "PowerSocket".to_string());
        assert_eq!(device.get_value(), 0.0);
        assert_eq!(device.get_state(), "OFF".to_string());
    }

    #[test]
    fn test_turn_on_off_power_socket() {
        let mut device = SmartDevice::power_socket(120.0);

        device.turn_on();
        assert_eq!(device.get_state(), "ON".to_ascii_uppercase());
        assert_eq!(device.get_value(), 120.0);

        device.turn_off();
        assert_eq!(device.get_state(), "OFF".to_ascii_uppercase());
        assert_eq!(device.get_value(), 0.0);
    }
}
