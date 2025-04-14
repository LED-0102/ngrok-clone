pub mod user;
mod client;

use actix_web::{web};
use crate::config::get_config;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    // WebSocket route inside /v1 scope
    cfg.service(
        web::scope("/v1")
            .guard(guard::fn_guard(|req| req.connection_info().host() == get_config("APP_DOMAIN")))
            .route("/ws", web::get().to(client::client_route))
    );

    cfg.default_service(web::route().to(user::user_route));

}
