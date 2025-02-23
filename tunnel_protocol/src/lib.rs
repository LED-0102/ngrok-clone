pub mod http_request;
pub mod http_response;
pub mod message;

use crate::http_request::HttpRequestWrapper;
use crate::http_response::HttpResponseWrapper;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageProtocol<'a> {
    HTTPRequest(HttpRequestWrapper<'a>),
    HTTPResponse(HttpResponseWrapper<'a>),
    WebSocketMessage,
    WebSocketConnect
}


impl<'a> MessageProtocol<'a> {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_string(s: &'a str) -> Result<MessageProtocol<'a>, serde_json::Error> {
        serde_json::from_str(s)
    }

}
