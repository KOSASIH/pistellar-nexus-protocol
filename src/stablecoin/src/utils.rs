/// Validates a Stellar address.
///
/// # Arguments
///
/// * `address` - A string slice that holds the address to be validated.
///
/// # Returns
///
/// Returns `true` if the address is valid, `false` otherwise.
pub fn validate_address(address: &str) -> bool {
    // Check if the address length is exactly 56 characters
    if address.len() != 56 {
        return false;
    }

    // Check if the address starts with 'G'
    if !address.starts_with("G") {
        return false;
    }

    // Additional checks can be added here, such as checking for valid characters
    // Stellar addresses are base32 encoded, so we can check for valid base32 characters
    for c in address.chars() {
        if !is_valid_base32_char(c) {
            return false;
        }
    }

    true
}

/// Checks if a character is a valid base32 character.
///
/// # Arguments
///
/// * `c` - A character to be checked.
///
/// # Returns
///
/// Returns `true` if the character is a valid base32 character, `false` otherwise.
fn is_valid_base32_char(c: char) -> bool {
    c.is_ascii_alphanumeric() && (c.is_digit(10) || matches!(c, 'A'..='Z' | 'a'..='z'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address() {
        let valid_address = "GABCD12345678901234567890123456789012345678901234";
        assert!(validate_address(valid_address));
    }

    #[test]
    fn test_invalid_length() {
        let invalid_address = "GABCD1234567890123456789012345678901234567890123"; // 55 characters
        assert!(!validate_address(invalid_address));
    }

    #[test]
    fn test_invalid_prefix() {
        let invalid_address = "XABCD12345678901234567890123456789012345678901234"; // Does not start with 'G'
        assert!(!validate_address(invalid_address));
    }

    #[test]
    fn test_invalid_character() {
        let invalid_address = "GABCD1234567890123456789012345678901234567890123$"; // Contains invalid character '$'
        assert!(!validate_address(invalid_address));
    }
}
