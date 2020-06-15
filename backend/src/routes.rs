use crate::handlers::{
    health::get_health,
    user::{get_user, get_users},
};

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        .service(
            web::scope("/api/v1")
                // USER roles
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("", web::get().to(get_users)),
                ),
        );
}
