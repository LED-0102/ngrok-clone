mod actors;
mod api;
mod request_manager;

use actors::server;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};
use std::collections::HashMap;
use actix::*;
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, dev::Service
};
use actix_web_actors::ws;
use once_cell::sync::Lazy;
use crate::actors::client;
use crate::request_manager::RequestState;

static GLOBAL_REQUEST_STATE: Lazy<Arc<RequestState>> = Lazy::new(|| Arc::new(RequestState::new()));

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server.rs at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                println!("{} {}", req.method(), req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .app_data(web::PayloadConfig::new(usize::MAX))
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .configure(api::api_config)
            .wrap(Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
