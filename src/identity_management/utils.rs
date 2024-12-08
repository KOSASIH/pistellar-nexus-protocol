use rand::Rng;

pub fn generate_random_id(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

pub fn serialize_identity(identity: &super::identity::Identity) -> String {
    serde_json::to_string(identity).unwrap()
}

pub fn deserialize_identity(data: &str) -> super::identity::Identity {
    serde_json::from_str(data).unwrap()
}
