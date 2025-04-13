pub mod user;
mod client;

use actix_web::{web};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    // WebSocket route inside /v1 scope
    cfg.service(
        web::scope("/v1")
            .route("/ws", web::get().to(client::client_route))
    );

    // Define a fallback route — ONLY triggered when nothing else matched
    cfg.default_service(
        web::route().to(user::user_route)
    );
}
