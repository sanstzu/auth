use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    id: String,
    expiry: OffsetDateTime,
}

impl JwtClaims {
    pub fn new(id: String, expiry: OffsetDateTime) -> Self {
        JwtClaims { id, expiry }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_expiry(&self) -> &OffsetDateTime {
        &self.expiry
    }
}
