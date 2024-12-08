// iot_integration/device_manager.rs

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    id: String,
    name: String,
    status: DeviceStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Error(String),
}

pub struct DeviceManager {
    devices: Arc<Mutex<HashMap<String, Device>>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_device(&self, id: String, name: String) {
        let device = Device {
            id: id.clone(),
            name,
            status: DeviceStatus::Offline,
        };
        self.devices.lock().unwrap().insert(id, device);
    }

    pub fn remove_device(&self, id: &str) {
        self.devices.lock().unwrap().remove(id);
    }

    pub fn update_device_status(&self, id: &str, status: DeviceStatus) {
        if let Some(device) = self.devices.lock().unwrap().get_mut(id) {
            device.status = status;
        }
    }

    pub fn get_device(&self, id: &str) -> Option<Device> {
        self.devices.lock().unwrap().get(id).cloned()
    }

    pub fn list_devices(&self) -> Vec<Device> {
        self.devices.lock().unwrap().values().cloned().collect()
    }

    pub fn communicate_with_device(&self, id: &str, message: &str) -> Result<String, String> {
        if let Some(device) = self.devices.lock().unwrap().get(id) {
            match device.status {
                DeviceStatus::Online => Ok(format!("Message sent to {}: {}", device.name, message)),
                DeviceStatus::Offline => Err(format!("Device {} is offline.", device.name)),
                DeviceStatus::Error(ref err) => Err(format!("Device {} encountered an error: {}", device.name, err)),
            }
        } else {
            Err("Device not found.".to_string())
        }
    }
}
