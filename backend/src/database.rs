use crate::models::user::{create_random_user, User};
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs, path};

// Stub user data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStorage(pub Vec<User>);

lazy_static! {
    pub static ref USER_STORAGE: UserStorage = load_user_storage();
}

fn create_user_storage(n: u32) -> UserStorage {
    let mut users: Vec<User> = Vec::new();
    for _ in 0..n {
        let u = create_random_user();
        info!("Adding user {:?}", u);
        users.push(u);
    }
    UserStorage(users)
}

fn load_user_storage() -> UserStorage {
    let path = "users.json";
    if path::Path::new(path).exists() {
        info!("User database found in {}, loading", path);
        let lines = fs::read_to_string(path).expect(&format!("Failed to read {}", path));
        serde_json::from_str(&lines).unwrap()
    } else {
        info!("User database not found, creating default in {}", path);
        let number_of_users = 5;
        let users = create_user_storage(number_of_users);
        let string = serde_json::to_string(&users).unwrap();
        let mut file = fs::File::create(path).unwrap();
        file.write_all(&string.into_bytes()).unwrap();
        users
    }
}

pub fn add_user_storage(cfg: &mut web::ServiceConfig) {
    info!("Adding user storage...");
    cfg.data(USER_STORAGE.clone());
}
