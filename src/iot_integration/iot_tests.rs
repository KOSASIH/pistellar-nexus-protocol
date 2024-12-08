// iot_integration/iot_tests.rs

#[cfg(test)]
mod tests {
    use super::device_manager::{DeviceManager, DeviceStatus};

    #[test]
    fn test_add_device() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.name, "Temperature Sensor");
        assert_eq!(device.status, DeviceStatus::Offline);
    }

    #[test]
    fn test_remove_device() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.remove_device("1");
        assert!(manager.get_device("1").is_none());
    }

    #[test]
    fn test_update_device_status() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Online);
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.status, DeviceStatus::Online);
    }

    #[test]
    fn test_communicate_with_device() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Online);
        let response = manager.communicate_with_device("1", "Get Temperature");
        assert_eq!(response.unwrap(), "Message sent to Temperature Sensor: Get Temperature");
    }

    #[test]
    fn test_communicate_with_offline_device() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        let response = manager.communicate_with_device("1", "Get Temperature");
        assert_eq!(response.unwrap_err(), "Device Temperature Sensor is offline.");
    }

    #[test]
    fn test_communicate_with_device_error() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Error("Sensor malfunction".to_string()));
        let response = manager.communicate_with_device("1", "Get Temperature");
        assert_eq!(response.unwrap_err(), "Device Temperature Sensor encountered an error: Sensor malfunction");
    }

    #[test]
    fn test_list_devices() {
        let manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.add_device("2".to_string(), "Humidity Sensor".to_string());
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].name, "Temperature Sensor");
        assert_eq!(devices[1].name, "Humidity Sensor");
    }

    #[test]
    fn test_get_nonexistent_device() {
        let manager = DeviceManager::new();
        assert!(manager.get_device("nonexistent").is_none());
    }
}
