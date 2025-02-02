use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::{http, Message};
use tunnel_protocol::http_request::HttpRequestWrapper;
use tunnel_protocol::http_response::HttpResponseWrapper;
use tunnel_protocol::message::Message as TunnelMessage;
use tunnel_protocol::MessageProtocol;

/// Handles incoming WebSocket messages
pub async fn handle_message (msg: TunnelMessage) -> Option<TunnelMessage> {
    let protocol_message: MessageProtocol = serde_json::from_str(&msg.msg).unwrap();

    match protocol_message {
        MessageProtocol::HTTPRequest(req) => {
            let response = handle_http_request(req).await;

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

pub async fn handle_http_request (req: HttpRequestWrapper) -> String {
    let uri = format!("{}{}", "http://localhost:8080/", req.uri);

    let client = reqwest::Client::new();

    let mut request_builder = client.request(http::method::Method::from(req.method.as_ref()), &uri);

    for (key, value) in req.headers.iter() {
        request_builder = request_builder.header(key.as_ref(), value.as_ref());
    }

    let response = match request_builder.body(reqwest::Body::from(req.body)).send().await {
        Ok(response) => response,
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

    HttpResponseWrapper::from_reqwest_response(response).await

}