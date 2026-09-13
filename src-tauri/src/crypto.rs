use anyhow::Result;

const SERVICE_NAME: &str = "luna-desktop";

pub struct SecureStorage {
    available: bool,
}

impl SecureStorage {
    pub fn new() -> Self {
        let available = keyring::Entry::new(SERVICE_NAME, "check")
            .and_then(|e| e.get_password())
            .is_ok();
        Self { available }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn store(&self, key: &str, value: &str) -> Result<()> {
        if self.available {
            keyring::Entry::new(SERVICE_NAME, key)?
                .set_password(value)
                .map_err(|e| anyhow::anyhow!("keyring store failed: {}", e))?;
        }
        Ok(())
    }

    pub fn retrieve(&self, key: &str) -> Result<String> {
        if self.available {
            return keyring::Entry::new(SERVICE_NAME, key)?
                .get_password()
                .map_err(|e| anyhow::anyhow!("keyring retrieve failed: {}", e));
        }
        Err(anyhow::anyhow!("keyring unavailable"))
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        if self.available {
            keyring::Entry::new(SERVICE_NAME, key)?
                .delete_password()
                .map_err(|e| anyhow::anyhow!("keyring delete failed: {}", e))?;
        }
        Ok(())
    }
}

pub fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "*".repeat(key.len());
    }
    let start = &key[..4];
    let end = &key[key.len() - 4..];
    format!("{}•{}•{}", start, "*".repeat(key.len() - 8), end)
}

const XOR_KEY: &[u8] = b"luna-desktop-secure";

pub fn obfuscate(input: &str) -> String {
    let bytes = input.as_bytes();
    let obfuscated: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ XOR_KEY[i % XOR_KEY.len()])
        .collect();
    hex::encode(obfuscated)
}

pub fn deobfuscate(hex_input: &str) -> String {
    let bytes = match hex::decode(hex_input) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    let deobfuscated: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ XOR_KEY[i % XOR_KEY.len()])
        .collect();
    String::from_utf8_lossy(&deobfuscated).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_short_key() {
        assert_eq!(mask_key("abc"), "***");
    }

    #[test]
    fn mask_long_key() {
        let masked = mask_key("sk-or-v1-1234567890abcdef");
        assert!(masked.starts_with("sk-o"));
        assert!(masked.ends_with("cdef"));
        assert!(masked.contains('•'));
    }

    #[test]
    fn mask_8_chars() {
        assert_eq!(mask_key("12345678"), "12345678");
    }

    #[test]
    fn mask_9_chars() {
        let m = mask_key("123456789");
        assert!(m.starts_with("1234"));
        assert!(m.ends_with("6789"));
        assert!(m.contains('•'));
    }

    #[test]
    fn obfuscate_deobfuscate_roundtrip() {
        let original = "sk-or-v1-1234567890abcdef";
        let obfuscated = obfuscate(original);
        assert_ne!(obfuscated, original);
        let deobfuscated = deobfuscate(&obfuscated);
        assert_eq!(deobfuscated, original);
    }
}
