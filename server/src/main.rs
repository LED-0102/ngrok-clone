mod actors;
mod api;
mod request_manager;
mod config;

use actors::server;

use crate::actors::client;
use crate::request_manager::RequestState;
use actix::*;
use actix_web::{
    dev::Service, middleware::Logger, web, App, HttpResponse, HttpServer, Responder
};
use once_cell::sync::Lazy;
use std::sync::{
    atomic::AtomicUsize,
    Arc,
};

static GLOBAL_REQUEST_STATE: Lazy<Arc<RequestState>> = Lazy::new(|| Arc::new(RequestState::new()));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let app_state = Arc::new(AtomicUsize::new(0));

    let server = server::ChatServer::new(app_state.clone()).start();

    let port: u16 = config::get_config("PORT");

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

            .route("/hello", web::get().to(hello_world)) // This is your new route
    })
        .workers(2)
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<html><body><h1>Hello World</h1></body></html>")
}
