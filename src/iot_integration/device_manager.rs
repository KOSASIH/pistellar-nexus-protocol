use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use log::{info, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    id: String,
    name: String,
    status: DeviceStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeviceStatus {
    Online,
    Offline,
    Error(String),
}

pub struct DeviceManager {
    devices: Arc<Mutex<HashMap<String, Device>>>,
}

impl DeviceManager {
    /// Creates a new instance of DeviceManager.
    pub fn new() -> Self {
        DeviceManager {
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a new device to the manager.
    pub fn add_device(&self, id: String, name: String) {
        let device = Device {
            id: id.clone(),
            name,
            status: DeviceStatus::Offline,
        };
        self.devices.lock().unwrap().insert(id, device);
        info!("Device added: {}", id);
    }

    /// Removes a device from the manager.
    pub fn remove_device(&self, id: &str) {
        if self.devices.lock().unwrap().remove(id).is_some() {
            info!("Device removed: {}", id);
        } else {
            error!("Attempted to remove non-existent device: {}", id);
        }
    }

    /// Updates the status of a device.
    pub fn update_device_status(&self, id: &str, status: DeviceStatus) {
        if let Some(device) = self.devices.lock().unwrap().get_mut(id) {
            device.status = status;
            info!("Device status updated: {} -> {:?}", id, status);
        } else {
            error!("Attempted to update status of non-existent device: {}", id);
        }
    }

    /// Retrieves a device by its ID.
    pub fn get_device(&self, id: &str) -> Option<Device> {
        self.devices.lock().unwrap().get(id).cloned()
    }

    /// Lists all devices managed by the DeviceManager.
    pub fn list_devices(&self) -> Vec<Device> {
        self.devices.lock().unwrap().values().cloned().collect()
    }

    /// Communicates with a device and sends a message.
    pub fn communicate_with_device(&self, id: &str, message: &str) -> Result<String, String> {
        if let Some(device) = self.devices.lock().unwrap().get(id) {
            match device.status {
                DeviceStatus::Online => {
                    info!("Message sent to {}: {}", device.name, message);
                    Ok(format!("Message sent to {}: {}", device.name, message))
                },
                DeviceStatus::Offline => {
                    let err_msg = format!("Device {} is offline.", device.name);
                    error!("{}", err_msg);
                    Err(err_msg)
                },
                DeviceStatus::Error(ref err) => {
                    let err_msg = format!("Device {} encountered an error: {}", device.name, err);
                    error!("{}", err_msg);
                    Err(err_msg)
                },
            }
        } else {
            let err_msg = "Device not found.".to_string();
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
        }
