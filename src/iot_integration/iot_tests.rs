#[cfg(test)]
mod tests {
    use super::device_manager::{DeviceManager, DeviceStatus};
    use tokio; // For asynchronous testing

    #[tokio::test]
    async fn test_add_device() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.name, "Temperature Sensor");
        assert_eq!(device.status, DeviceStatus::Offline);
    }

    #[tokio::test]
    async fn test_remove_device() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.remove_device("1");
        assert!(manager.get_device("1").is_none());
    }

    #[tokio::test]
    async fn test_update_device_status() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Online).await.unwrap();
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.status, DeviceStatus::Online);
    }

    #[tokio::test]
    async fn test_communicate_with_device() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Online).await.unwrap();
        let response = manager.communicate_with_device("1", "Get Temperature").await;
        assert_eq!(response.unwrap(), "Message sent to Temperature Sensor: Get Temperature");
    }

    #[tokio::test]
    async fn test_communicate_with_offline_device() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        let response = manager.communicate_with_device("1", "Get Temperature").await;
        assert_eq!(response.unwrap_err(), "Device Temperature Sensor is offline.");
    }

    #[tokio::test]
    async fn test_communicate_with_device_error() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Error("Sensor malfunction".to_string())).await.unwrap();
        let response = manager.communicate_with_device("1", "Get Temperature").await;
        assert_eq!(response.unwrap_err(), "Device Temperature Sensor encountered an error: Sensor malfunction");
    }

    #[tokio::test]
    async fn test_list_devices() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.add_device("2".to_string(), "Humidity Sensor".to_string());
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].name, "Temperature Sensor");
        assert_eq!(devices[1].name, "Humidity Sensor");
    }

    #[tokio::test]
    async fn test_get_nonexistent_device() {
        let mut manager = DeviceManager::new();
        assert!(manager.get_device("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_device_status_transition() {
        let mut manager = DeviceManager::new();
        manager.add_device("1".to_string(), "Temperature Sensor".to_string());
        manager.update_device_status("1", DeviceStatus::Online).await.unwrap();
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.status, DeviceStatus::Online);

        manager.update_device_status("1", DeviceStatus::Offline).await.unwrap();
        let device = manager.get_device("1").unwrap();
        assert_eq!(device.status, DeviceStatus::Offline);
    }
    }
