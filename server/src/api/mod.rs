pub mod user;
mod client;

use actix_web::{web};

pub fn api_config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/ws", web::get().to(client::client_route))
            .route("/{tail:.*}", web::get().to(user::user_route))
    );
}