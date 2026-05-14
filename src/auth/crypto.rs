use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use sha2::{Digest, Sha256};

use crate::errors::CryptoError;

fn derive_key() -> [u8; 32] {
    let machine_id = whoami::fallible::hostname().unwrap_or_else(|_| "unknown-host".to_owned());

    let salt = b"kaneo-cli-v1-key-derivation-salt";
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(machine_id.as_bytes());

    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt(plaintext: &str) -> Result<String, CryptoError> {
    let key_bytes = derive_key();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext =
        cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| CryptoError::Encrypt {
                source: anyhow::anyhow!("{e}"),
            })?;

    let mut combined = Vec::with_capacity(nonce.len() + ciphertext.len());
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&combined))
}

pub fn decrypt(encoded: &str) -> Result<String, CryptoError> {
    let combined = BASE64.decode(encoded).map_err(|e| CryptoError::Decrypt {
        source: anyhow::anyhow!("failed to decode base64: {e}"),
    })?;

    if combined.len() < 12 {
        return Err(CryptoError::Decrypt {
            source: anyhow::anyhow!("encrypted data too short"),
        });
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let key_bytes = derive_key();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::Decrypt {
            source: anyhow::anyhow!("{e}"),
        })?;

    String::from_utf8(plaintext).map_err(|e| CryptoError::Decrypt {
        source: anyhow::anyhow!("decrypted data is not valid UTF-8: {e}"),
    })
}

#[cfg(test)]
mod crypto_tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let original = "kp_abc123secretkey";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn different_ciphertexts() {
        let plain = "test-key";
        let a = encrypt(plain).unwrap();
        let b = encrypt(plain).unwrap();
        assert_ne!(a, b); // different nonces
    }
}
