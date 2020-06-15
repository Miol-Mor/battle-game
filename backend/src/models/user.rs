use crate::auth::hash;
use crate::database::UserStorage;
use crate::errors::ApiError;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub handle: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

pub fn create_random_user() -> User {
    User {
        id: Uuid::new_v4().to_string(),
        handle: "test_user".to_string(),
        email: "test@test.com".to_string(),
        password: hash("tbd"),
        created_at: Utc::now().naive_utc(),
    }
}
