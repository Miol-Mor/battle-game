use crate::database::UserStorage;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::user::{find, get_all, User};
use actix_web::web::{block, Data, Json, Path};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub handle: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse(pub Vec<UserResponse>);

impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            handle: user.handle.to_string(),
            email: user.email.to_string(),
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> UsersResponse {
        UsersResponse(users.into_iter().map(|user| user.into()).collect())
    }
}

/// Api
/// Get user
pub async fn get_user(
    data: Data<UserStorage>,
    user_id: Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    info!("User id: {}", *user_id);
    let user = block(move || {
        let user = find(&data, *user_id);
        match user {
            Some(u) => Ok(u),
            None => Err(ApiError::NotFound("user".to_string())),
        }
    })
    .await?;
    respond_json(user.into())
}

/// Get all users
pub async fn get_users(data: Data<UserStorage>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || Ok(get_all(&data))).await?;
    respond_json(users.into())
}
