pub mod device;
pub mod power_socket;
pub mod thermometer;

use crate::device::Device;
use crate::power_socket::PowerSocket;
use crate::thermometer::Thermometer;
use std::collections::HashMap;
use std::ffi::c_uint;
use std::os::raw::c_char;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DEVICE_REGISTRY: RwLock<HashMap<c_uint, Box<dyn Device>>> =
        RwLock::new(HashMap::new());
}

#[repr(C)]
pub enum DeviceType {
    PowerSocket,
    Thermometer,
}

#[repr(C)]
pub struct DeviceContext {
    pub device_type: DeviceType,
    pub value: f64,
    pub is_on: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn new_device(device_type: DeviceType, value: f64) -> c_uint {
    let device: Box<dyn Device> = match device_type {
        DeviceType::PowerSocket => Box::new(PowerSocket::new(value)),
        DeviceType::Thermometer => Box::new(Thermometer::new(value)),
    };

    match DEVICE_REGISTRY.write() {
        Ok(mut registry) => {
            let id = registry.len() as c_uint;
            registry.insert(id, device);
            id
        }
        Err(_) => 0u32,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn device_on(id: c_uint) -> bool {
    let mut registry = DEVICE_REGISTRY.write().unwrap();
    if let Some(device) = registry.get_mut(&id) {
        device.on();
        return true;
    }
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn device_off(id: c_uint) -> bool {
    let mut registry = DEVICE_REGISTRY.write().unwrap();
    if let Some(device) = registry.get_mut(&id) {
        device.off();
        return true;
    }
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn device_get_value(id: c_uint) -> f64 {
    let registry = DEVICE_REGISTRY.read().unwrap();
    registry.get(&id).map(|d| d.get_value()).unwrap_or(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn device_get_name(id: c_uint, buffer: *mut c_char, len: usize) -> usize {
    let registry = DEVICE_REGISTRY.read().unwrap();
    let name = registry
        .get(&id)
        .map(|d| d.get_name())
        .unwrap_or("Unknown".to_string());
    copy_str_to_c(name, buffer, len)
}

#[unsafe(no_mangle)]
pub extern "C" fn device_state(id: c_uint, buffer: *mut c_char, len: usize) -> usize {
    let registry = DEVICE_REGISTRY.read().unwrap();
    let state = registry
        .get(&id)
        .map(|d| d.get_state())
        .unwrap_or("Unknown".to_string());
    copy_str_to_c(state, buffer, len)
}

fn copy_str_to_c(s: String, buffer: *mut c_char, len: usize) -> usize {
    let bytes = s.as_bytes();
    let n = (len.saturating_sub(1)).min(bytes.len());
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, n);
        *buffer.add(n) = 0;
    }
    bytes.len()
}
