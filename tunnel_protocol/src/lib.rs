use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct HttpRequest<'a> {
    method: Cow<'a, str>,
    uri: Cow<'a, str>,
    headers: HashMap<Cow<'a, str>, Cow<'a, [u8]>>,
    body: Cow<'a, [u8]>,
    request_id: Cow<'a, str>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageProtocol<'a> {
    Connect,
    Disconnect,
    HTTPRequest(HttpRequest<'a>),
    HTTPResponse,
    WebSocketConnect,
    WebSocketDisconnect,
    WebSocketMessage,
}

impl<'a> MessageProtocol<'a> {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_string(s: &'a str) -> Result<MessageProtocol<'a>, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn from_http_request<'b>(
        method: Cow<'b, str>,
        uri: Cow<'b, str>,
        headers: HashMap<Cow<'b, str>, Cow<'b, [u8]>>,
        body: Cow<'b, [u8]>,
        request_id: Cow<'b, str>,
    ) -> MessageProtocol<'b> {
        MessageProtocol::HTTPRequest(HttpRequest {
            method,
            uri,
            headers,
            body,
            request_id,
        })
    }
}
