use std::borrow::Cow;
use std::collections::HashMap;
use actix_web::{HttpRequest, web};
use serde_derive::{Deserialize, Serialize};
use futures::StreamExt;
use crate::MessageProtocol;

#[derive(Serialize, Deserialize)]
pub struct HttpRequestWrapper<'a> {
    pub method: Cow<'a, str>,
    pub uri: Cow<'a, str>,
    pub headers: HashMap<Cow<'a, str>, Cow<'a, [u8]>>,
    pub body: Cow<'a, [u8]>,
}

impl<'a> MessageProtocol<'a> {
    pub async fn from_actix_request(req: &HttpRequest, mut payload: web::Payload, tail: &str) -> String {
        let mut body = Vec::new();
        while let Some(chunk) = payload.next().await {
            if let Ok(bytes) = chunk {
                body.extend_from_slice(&bytes);
            }
        }

        let headers: HashMap<Cow<str>, Cow<[u8]>> = req.headers()
            .iter()
            .map(|(key, value)| (Cow::from(key.as_str()), Cow::from(value.as_bytes())))
            .collect();

        let http_request = MessageProtocol::HTTPRequest(HttpRequestWrapper {
            method: Cow::from(req.method().as_str()),
            uri: Cow::from(tail),
            headers,
            body: Cow::from(body.as_slice()),
        });

        http_request.to_string().unwrap()
    }
}
