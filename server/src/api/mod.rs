pub mod user;
mod client;

use actix_web::{web, guard};
use crate::config::get_config;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    // WebSocket route inside /v1 scope
    cfg.service(
        web::scope("/v1")
            .guard(guard::fn_guard(|req| req.head().uri.host().unwrap_or("") == get_config::<String>("APP_DOMAIN")))
            .route("/ws", web::get().to(client::client_route))
    );

    cfg.default_service(web::route().to(user::user_route));

}
