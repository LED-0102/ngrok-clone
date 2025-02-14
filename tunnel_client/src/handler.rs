use tokio_tungstenite::tungstenite::{http};
use tunnel_protocol::http_request::HttpRequestWrapper;
use tunnel_protocol::http_response::HttpResponseWrapper;
use tunnel_protocol::message::Message as TunnelMessage;
use tunnel_protocol::MessageProtocol;
use std::borrow::Cow;
use std::collections::HashMap;
use reqwest::Method;
use std::str::FromStr;
use crate::config::Config;

/// Handles incoming WebSocket messages
pub async fn handle_message (msg: TunnelMessage, config: &Config) -> Option<TunnelMessage> {
    let mut protocol_message: MessageProtocol = serde_json::from_str(&msg.msg).unwrap();

    match protocol_message {
        MessageProtocol::HTTPRequest(req) => {
            let response = handle_http_request(req, config).await;
            println!("Response: {}", response);
            return Some(TunnelMessage {
                msg: response,
                id: msg.id,
            });
        }
        MessageProtocol::HTTPResponse(_) => {}
        MessageProtocol::WebSocketMessage => {}
        MessageProtocol::WebSocketConnect => {}
    }

    None
}

pub async fn handle_http_request (req: HttpRequestWrapper<'_>, config: &Config) -> String {
    let uri = format!("{}{}{}{}", "http://localhost:", config.local_port, "/", req.uri);
    println!("Sending request to: {}", &uri);
    let client = reqwest::Client::new();

    let mut request_builder = client.request(Method::from_str(req.method.as_ref()).unwrap(), &uri);

    for (key, value) in req.headers.iter() {
        request_builder = request_builder.header(key.as_ref(), value.as_ref());
    }

    let body = req.body.to_vec();

    let response = match request_builder.body(body).send().await {
        Ok(response) => {
            println!("Received Response");
            response
        },
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
            let resp =  HttpResponseWrapper {
                status_code: 500,
                headers: HashMap::new(),
                body: Cow::from(b"Internal Server Error".to_vec()),
            };

            return serde_json::to_string(&resp).unwrap();
        }
    };



    MessageProtocol::from_reqwest_response(response).await

}