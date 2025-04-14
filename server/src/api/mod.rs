pub mod user;
mod client;

use actix_web::{web, guard};
use crate::config::get_config;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    // WebSocket route inside /v1 scope
    cfg.service(
        web::scope("/v1")
            .guard(guard::fn_guard(|req| {

                let app_domain: String = get_config("APP_DOMAIN");

                let host = req.head().headers.get("host").and_then(|h| h.to_str().ok()).unwrap_or("");

                println!("{host}");

                host == app_domain
            }

            ))
            .route("/ws", web::get().to(client::client_route))
    );

    cfg.default_service(web::route().to(user::user_route));

}
