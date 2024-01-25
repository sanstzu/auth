use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;

pub enum UserCreateError {
    DatabaseError,
    HashingError,
    DuplicateUser,
}

pub async fn user_create(
    db: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), UserCreateError> {
    let uuid = Uuid::new_v4().to_string();
    let mut mut_user: User = User::new(uuid, username.to_string(), password.to_string());

    match mut_user.hash_self() {
        Err(_) => Err(UserCreateError::HashingError),
        Ok(_) => {
            match User::query_by_id(db, mut_user.get_id()).await {
                Err(_) => return Err(UserCreateError::DatabaseError),
                Ok(Some(_)) => return Err(UserCreateError::DuplicateUser),
                _ => (),
            }

            match User::add_user(db, &mut_user).await {
                Err(_) => Err(UserCreateError::DatabaseError),
                Ok(_) => Ok(()),
            }
        }
    }
}
