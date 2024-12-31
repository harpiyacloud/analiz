use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};

use crate::{encoding::base64, error::ApiError};

/// Encrypts the hashed password using `Argon2id`.
pub(crate) fn encrypt_hashed_password(hashed_password: &[u8], key: &[u8]) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(hashed_password, &salt).unwrap()
        .to_string();
    let ciphertext = crate::crypto::encrypt(password_hash.as_bytes(), key)?;
    Ok(base64::encode(ciphertext))
}

/// Encrypts the raw password using `Argon2id`.
pub(crate) fn encrypt_raw_password(raw_password: &[u8], key: &[u8]) -> Result<String, ApiError> {
    let hashed_password = base64::encode(crate::crypto::digest(raw_password));
    encrypt_hashed_password(hashed_password.as_bytes(), key)
}

/// Verifies the hashed password using `Argon2id`.
pub(crate) fn verify_hashed_password(
    hashed_password: &[u8],
    encrypted_password: &[u8],
    key: &[u8],
) -> Result<bool, ApiError> {
    let ciphertext = base64::decode(encrypted_password)?;
    let password_hash = crate::crypto::decrypt(&ciphertext, key)?;
    let password_hash_str = String::from_utf8_lossy(&password_hash);
    let parsed_hash = PasswordHash::new(&password_hash_str)?;
    Argon2::default().verify_password(hashed_password, &parsed_hash)?;
    Ok(true)
}

/// Verifies the raw password using `Argon2id`.
pub(crate) fn verify_raw_password(
    raw_password: &[u8],
    encrypted_password: &[u8],
    key: &[u8],
) -> Result<bool, ApiError> {
    let hashed_password = base64::encode(crate::crypto::digest(raw_password));
    verify_hashed_password(hashed_password.as_bytes(), encrypted_password, key)
}

pub fn encrypt_password(password: &str) -> Result<String, String> {
    let key = secret_key();
    let password = password.as_bytes();
    if base64::decode(password).is_ok_and(|bytes| bytes.len() == 256) {
        encrypt_hashed_password(password, &key)
            .map_err(|err| format!("fail to encrypt hashed password: {:#?}", err))
    } else {
        encrypt_raw_password(password, &key)
            .map_err(|err| format!("fail to encrypt raw password: {:#?}", err))
    }
}

pub fn verify_password(password: &str, encrypted_password: &str) -> Result<bool, String> {
    let key = secret_key();
    let password = password.as_bytes();
    let encrypted_password = encrypted_password.as_bytes();
    if base64::decode(password).is_ok_and(|bytes| bytes.len() == 256) {
        verify_hashed_password(password, encrypted_password, &key)
            .map_err(|err| format!("fail to verify hashed password: {:#?}", err))
    } else {
        verify_raw_password(password, encrypted_password, &key)
            .map_err(|err| format!("fail to verify raw password: {:#?}", err))
    }
}

/// Secret key.
fn secret_key() -> [u8; 64] {
    let checksum = {
        let secret = "HARPIYA::SECRET".as_bytes();
        crate::crypto::digest(secret)
    };
    let info = "HARPIYA:ORM";
    crate::crypto::derive_key(info, &checksum)
}
