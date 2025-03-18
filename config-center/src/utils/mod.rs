use anyhow::Result;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};

pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn read_file_to_string(path: &Path) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn write_string_to_file(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)?;
    Ok(())
}

pub fn is_valid_key(key: &str) -> bool {
    // Key format: namespace.key or key
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() > 2 {
        return false;
    }

    for part in parts {
        if part.is_empty() {
            return false;
        }
        if !part.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return false;
        }
    }

    true
}

pub fn encrypt_value(value: &str, key: &[u8]) -> Result<String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};

    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(b"unique nonce"); // In production, use a unique nonce
    let ciphertext = cipher.encrypt(nonce, value.as_bytes()).map_err(|e| anyhow::anyhow!(e))?;
    Ok(STANDARD.encode(ciphertext))
}

pub fn decrypt_value(encrypted: &str, key: &[u8]) -> Result<String> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};

    let cipher = Aes256Gcm::new(key.into());
    let nonce = Nonce::from_slice(b"unique nonce"); // In production, use a unique nonce
    let ciphertext = STANDARD.decode(encrypted)?;
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| anyhow::anyhow!(e))?;
    Ok(String::from_utf8(plaintext)?)
}

pub fn calculate_hash(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_uuid_generation() {
        let uuid = generate_uuid();
        assert!(!uuid.to_string().is_empty());
    }

    #[test]
    fn test_timestamp() {
        let timestamp = get_current_timestamp();
        assert!(timestamp > 0);
        assert!(!format_timestamp(timestamp).is_empty());
    }

    #[test]
    fn test_compression() {
        let data = b"test data";
        let compressed = compress_data(data).unwrap();
        let decompressed = decompress_data(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_file_operations() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = "test content";

        write_string_to_file(&file_path, content).unwrap();
        let read_content = read_file_to_string(&file_path).unwrap();
        assert_eq!(content, read_content);
    }

    #[test]
    fn test_key_validation() {
        assert!(is_valid_key("valid.key"));
        assert!(is_valid_key("valid-key"));
        assert!(is_valid_key("valid_key"));
        assert!(is_valid_key("valid"));
        assert!(!is_valid_key("invalid.key."));
        assert!(!is_valid_key(".invalid"));
        assert!(!is_valid_key("invalid@key"));
    }

    #[test]
    fn test_encryption() {
        let key = b"0123456789abcdef0123456789abcdef"; // 32 bytes
        let value = "test value";
        let encrypted = encrypt_value(value, key).unwrap();
        let decrypted = decrypt_value(&encrypted, key).unwrap();
        assert_eq!(value, decrypted);
    }

    #[test]
    fn test_hash() {
        let data = b"test data";
        let hash = calculate_hash(data);
        assert_eq!(hash.len(), 64);
    }
}
