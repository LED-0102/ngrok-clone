use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use actix_web::{HttpResponse as ActixHttpResponse, http::header};
use serde_derive::{Deserialize, Serialize};
use bytes::Bytes;
use reqwest::Response;
use crate::MessageProtocol;

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponseWrapper<'a> {
    pub status_code: u16,
    pub headers: HashMap<Cow<'a, str>, Cow<'a, [u8]>>,
    pub body: Cow<'a, [u8]>,
}

impl<'a> From<HttpResponseWrapper<'a>> for ActixHttpResponse {
    fn from(custom_response: HttpResponseWrapper<'a>) -> Self {
        let mut actix_response = ActixHttpResponse::build(
            actix_web::http::StatusCode::from_u16(custom_response.status_code)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        );

        for (key, value) in custom_response.headers {
            actix_response.append_header((
                header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                header::HeaderValue::from_bytes(value.as_ref()).unwrap(),
            ));
        }


        actix_response.body(Bytes::from(custom_response.body.to_vec()))
    }
}

impl<'a> FromStr for HttpResponseWrapper<'a> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(response) => Ok(response),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a> MessageProtocol<'a> {
    pub async fn from_reqwest_response(response: Response) -> String {
        let status_code = response.status().as_u16();
        let header_map = response.headers().clone();
        let headers: HashMap<Cow<str>, Cow<[u8]>> = header_map
            .iter()
            .map(|(key, value)| (Cow::from(key.as_str()), Cow::from(value.as_bytes())))
            .collect();

        let body_result = response.bytes().await;
        match body_result {
            Ok(body) => {
                let wrapper = MessageProtocol::HTTPResponse(HttpResponseWrapper {
                    status_code,
                    headers,
                    body: Cow::from(body.to_vec()),
                });
                serde_json::to_string(&wrapper).unwrap()
            }
            Err(_) => {
                let error_wrapper = MessageProtocol::HTTPResponse(HttpResponseWrapper {
                    status_code: 500,
                    headers: HashMap::new(),
                    body: Cow::from(b"Internal Server Error".to_vec()),
                });
                serde_json::to_string(&error_wrapper).unwrap()
            }
        }
    }
}

