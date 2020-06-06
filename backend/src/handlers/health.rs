use actix_web::{error::Error, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn get_health() -> Result<Json<HealthResponse>, Error> {
    Ok(Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_health() {
        let response = get_health().await.unwrap();
        let inner = response.into_inner();
        assert_eq!(inner.status, "ok".to_string());
        assert_eq!(inner.version, env!("CARGO_PKG_VERSION").to_string());
    }
}
