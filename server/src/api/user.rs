use std::str::FromStr;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use actix::{Addr};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::distr::Alphanumeric;
use rand::Rng;
use crate::actors::server;
use crate::request_manager::RequestState;
use tunnel_protocol::{MessageProtocol, http_response};
use crate::actors;

fn generate_unique_string() -> String {
    let random_string: String = rand::rng()
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
    let random_string: String = rand::rng()
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
    mut req: HttpRequest,
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

    let mut recv = match srv.send(server::AddRequest {
        request_id: request_id.clone(),
        client_id: service_id.clone(),
        request_state: r,
    })
        .await {
        Ok(s) => {s}
        Err(e) => {
            return Ok(HttpResponse::BadRequest().body(e.to_string()));
        }
    };

    if is_websocket {
        let user_id = generate_unique_string();

        todo!("Implement the WebSocket connection");

        ws::start(
            actors::user::UserActor {
                hb: Instant::now(),
                user_id,
                client: client.unwrap(),
            },
            &req,
            stream,
        )
    } else {
        let http_request = MessageProtocol::from_actix_request(&req, stream, &tail).await;

        srv.do_send(server::SendMessage {
            client_id: service_id.to_string(),
            message: http_request,
            request_id,
        });

        match recv {
            Some(rx) => {
                match rx.await {
                    Ok(response) => {
                        match http_response::HttpResponseWrapper::from_str(&response) {
                            Ok(res) => {
                                Ok(res.into())
                            }
                            Err(e) => {
                                Ok(HttpResponse::InternalServerError().body(e))
                            }
                        }
                    },
                    Err(e) => {
                        Ok(HttpResponse::InternalServerError().body(e.to_string()))
                    }
                }
            }
            None => {
                Ok(HttpResponse::BadRequest().body("Invalid URL"))
            }
        }
    }
}