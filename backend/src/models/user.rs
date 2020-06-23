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

pub fn find(storage: &UserStorage, id: Uuid) -> Option<User> {
    match storage
        .0
        .iter()
        .position(|user| user.id.eq(&id.to_string()))
    {
        Some(pos) => Some(storage.0[pos].clone()),
        None => None,
    }
}

pub fn get_all(storage: &UserStorage) -> Vec<User> {
    storage.0.clone()
}

#[cfg(test)]
mod test {
    use crate::models::user::create_random_user;

    #[test]
    fn new() {
        let u = create_random_user();
        println!("Adding user {:?}", u);
    }
}