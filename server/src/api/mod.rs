pub mod user;
mod client;

use actix_web::{web};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    // WebSocket route inside /v1 scope
    cfg.service(
        web::scope("/v1")
            .route("/ws", web::get().to(client::client_route))
    );

    // Global catch-all route for subdomain-based access
    cfg.route("/{tail:.*}", web::get().to(user::user_route));
}
