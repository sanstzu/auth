use sqlx::PgPool;

use crate::{app::auth::utils::hash::hash_verify, models::User};

pub enum LoginError {
    UserNotExist,
    PasswordIncorrect,
    DatabaseError,
    VerifyError,
}

pub async fn user_login(username: &str, password: &str, db: &PgPool) -> Result<User, LoginError> {
    let user_query = User::query_by_username(db, username).await;

    match user_query {
        Err(_) => Err(LoginError::DatabaseError),
        Ok(query) => match query {
            None => Err(LoginError::UserNotExist),
            Some(user) => match hash_verify(password, user.get_password()) {
                Err(_) => Err(LoginError::VerifyError),
                Ok(false) => Err(LoginError::PasswordIncorrect),
                Ok(true) => Ok(user),
            },
        },
    }
}
