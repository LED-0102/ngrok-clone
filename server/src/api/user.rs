use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::actors;
use crate::actors::client::ClientActor;
use crate::actors::server;
use crate::request_manager::RequestState;

fn generate_unique_string() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    format!("{random_string}-{timestamp}")
}

fn generate_unique_request_id() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    format!("{timestamp}-{random_string}")
}
pub async fn user_route (
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    path: web::Path<(String, String)>,
    request_state: web::Data<RequestState>
) -> Result<HttpResponse, Error> {
    let (service_id, tail) = path.into_inner();

    let key = srv.send(server::CheckKey {
        key: service_id.clone(),
    })
        .await
        .unwrap();

    if key == false {
        return Ok(HttpResponse::BadRequest()
            .body("Invalid URL"));
    } else {
        println!("User connected for session: {}", &service_id);
    }

    let client = srv.send(server::ClientAddr {
        client_id: service_id.clone(),
    })
        .await
        .unwrap();

    if client.is_none() {
        return Ok(HttpResponse::BadRequest()
            .body("Invalid URL"));
    }

    let is_websocket = req
        .headers()
        .get("Upgrade")
        .map(|value| value.as_bytes() == b"websocket")
        .unwrap_or(false);

    let request_id = generate_unique_request_id();
    let mut r = request_state.clone();

    if is_websocket {
        let user_id = generate_unique_string();
        // return ws::start(
        //     actors::user::UserActor {
        //         hb: Instant::now(),
        //         user_id,
        //         client: client.unwrap(),
        //     },
        //     &req,
        //     stream,
        // );
    } else {
        let mut recv = srv.send(server::AddRequest {
            request_id: request_id.clone(),
            client_id: service_id.clone(),
            request_state: r,
        })
            .await.unwrap();

        // Logic to send request to client

        // Receive response from client
        match recv {
            Some(rx) => {
                match rx.await {
                    Ok(response) => {
                        println!("Response from client: {}", response);
                    //     Forward the response to client
                    },
                    Err(e) => {
                        return Ok(HttpResponse::InternalServerError().body(e.into()));
                    }
                }
            }
            None => {
                return Ok(HttpResponse::BadRequest().body("Invalid URL"));
            }
        };
    }


    Ok(HttpResponse::Ok().body("Hello"))
}