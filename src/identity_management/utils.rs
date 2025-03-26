use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};
use std::fmt;

#[derive(Debug)]
pub enum IdentityError {
    SerializationError(SerdeError),
    DeserializationError(SerdeError),
    InvalidLength,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            IdentityError::DeserializationError(err) => write!(f, "Deserialization error: {}", err),
            IdentityError::InvalidLength => write!(f, "Invalid length for random ID"),
        }
    }
}

impl std::error::Error for IdentityError {}

pub fn generate_random_id(length: usize) -> Result<String, IdentityError> {
    if length == 0 {
        return Err(IdentityError::InvalidLength);
    }
    
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    let random_id: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    
    Ok(random_id)
}

pub fn serialize_identity(identity: &super::identity::Identity) -> Result<String, IdentityError> {
    serde_json::to_string(identity).map_err(IdentityError::SerializationError)
}

pub fn deserialize_identity(data: &str) -> Result<super::identity::Identity, IdentityError> {
    serde_json::from_str(data).map_err(IdentityError::DeserializationError)
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity; // Adjust the path as necessary
    use std::collections::HashMap;

    #[test]
    fn test_generate_random_id() {
        let random_id = generate_random_id(10).unwrap();
        assert_eq!(random_id.len(), 10);
        assert!(random_id.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_generate_random_id_zero_length() {
        let result = generate_random_id(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid length for random ID");
    }

    #[test]
    fn test_serialize_identity() {
        let mut metadata = HashMap::new();
        metadata.insert("email".to_string(), "test@example.com".to_string());
        let identity = Identity {
            id: "user1".to_string(),
            public_key: vec![1, 2, 3, 4, 5],
            metadata,
        };

        let serialized = serialize_identity(&identity).unwrap();
        assert!(serialized.contains("user1"));
        assert!(serialized.contains("test@example.com"));
    }

    #[test]
    fn test_deserialize_identity() {
        let json_data = r#"{"id":"user1","public_key":[1,2,3,4,5],"metadata":{"email":"test@example.com"}}"#;
        let identity: Identity = deserialize_identity(json_data).unwrap();
        assert_eq!(identity.id, "user1");
        assert_eq!(identity.metadata.get("email").unwrap(), "test@example.com");
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let result = deserialize_identity("invalid json");
        assert!(result.is_err());
    }
        }
