use crate::consts::CONFIG;
use crate::models::User;
use axum::BoxError;
use bcrypt::{hash_with_salt, verify};
use once_cell::sync::Lazy;

static SALT: Lazy<[u8; 16]> = Lazy::new(|| {
    let salt = CONFIG.get_auth_bcrypt_salt();
    let mut salt_bytes: Vec<u8> = salt.as_bytes().to_vec();
    salt_bytes.resize(16, 0);
    salt_bytes.as_slice().try_into().unwrap()
});
static COST: u32 = 12;

pub fn hash_string(string: &str) -> Result<String, BoxError> {
    let hash_parts = hash_with_salt(string, COST, *SALT)?;

    Ok(hash_parts.format_for_version(bcrypt::Version::TwoA))
}

pub fn hash_verify(string: &str, hash: &str) -> Result<bool, BoxError> {
    let equal = verify(string, hash)?;

    Ok(equal)
}

impl User {
    pub fn hash_self(&mut self) -> Result<(), ()> {
        let hash_res = hash_string(self.get_password());
        match hash_res {
            Err(_) => Err(()),
            Ok(hash_pass) => {
                self.set_password(hash_pass);
                Ok(())
            }
        }
    }
}
