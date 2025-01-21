mod actors;
mod api;

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
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use crate::actors::client;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server.rs at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .configure(api::api_config)
            .wrap(Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
