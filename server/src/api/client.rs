use std::collections::HashMap;
use std::time::Instant;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::actors;
use crate::actors::server;

pub async fn client_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .headers()
        .get("user_id")
        .and_then(|val| val.to_str().ok());

    println!("User ID: {:?}", user_id);

    match user_id {
        Some(user_id) => {
            let key = srv.send(server::CheckKey {
                key: user_id.to_string(),
            })
                .await
                .unwrap();

            if key == true {
                return Ok(HttpResponse::BadRequest()
                    .body("Invalid user_id"));
            } else {
                println!("Session connected for user_id: {}", user_id);
            }

            ws::WsResponseBuilder::new(
                actors::client::ClientActor {
                    hb: Instant::now(),
                    addr: srv.get_ref().clone(),
                    client_id: user_id.to_string(),
                    user_sessions: HashMap::new(),
                },
                &req,
                stream
            ).frame_size(10*1024*1024)
                .start()
        }
        None => {
            Ok(HttpResponse::BadRequest()
                .body("Missing user_id header"))
        }
    }
}