use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce, Key, AeadCore};

// This will be our encryption key (in production, this should be securely stored/derived)
pub const KEY_BYTES: [u8; 32] = [42; 32];

pub struct Encryption;

impl Encryption {
    // Encrypt a string and return the encrypted bytes and nonce
    pub fn encrypt_password(password: &str) -> (Vec<u8>, Vec<u8>) {
        let key = Key::<Aes256Gcm>::from_slice(&KEY_BYTES);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let encrypted_password = cipher
            .encrypt(&nonce, password.as_bytes())
            .expect("encryption failure!");

        (encrypted_password, nonce.to_vec())
    }

    // Decrypt encrypted bytes using the stored nonce
    pub fn decrypt_password(encrypted_password: &[u8], nonce: &[u8]) -> String {
        let key = Key::<Aes256Gcm>::from_slice(&KEY_BYTES);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce);

        let decrypted_bytes = cipher
            .decrypt(nonce, encrypted_password)
            .expect("decryption failure!");

        String::from_utf8(decrypted_bytes)
            .expect("Invalid UTF-8")
    }
}