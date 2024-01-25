use once_cell::sync::Lazy;
use std::env;

pub static CONFIG: Lazy<Consts> = Lazy::new(Consts::load_consts);

#[derive(Clone)]
pub struct Consts {
    pub auth_bcrypt_salt: String,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub database_connection: String,
}

impl Consts {
    fn load_consts() -> Self {
        Consts {
            auth_bcrypt_salt: env::var("AUTH_BCRYPT_SALT").expect("AUTH_BCRYPT_SALT env not found"),
            access_token_secret: env::var("ACCESS_TOKEN_SECRET")
                .expect("ACCESS_TOKEN_SECRET env not found"),
            refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
                .expect("REFRESH_TOKEN_SECRET not found"),
            database_connection: env::var("DATABASE_URL").expect("DATABASE_URL env not found"),
        }
    }

    pub fn get_auth_bcrypt_salt(&self) -> &str {
        &self.auth_bcrypt_salt
    }

    pub fn get_access_token_secret(&self) -> &str {
        &self.access_token_secret
    }

    pub fn get_refresh_token_secret(&self) -> &str {
        &self.refresh_token_secret
    }

    pub fn get_database_connection(&self) -> &str {
        &self.database_connection
    }
}
