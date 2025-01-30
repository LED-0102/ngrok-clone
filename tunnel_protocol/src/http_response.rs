use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use actix_web::{HttpResponse as ActixHttpResponse, http::header};
use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponseWrapper<'a> {
    pub status_code: u16,
    pub status_message: Cow<'a, str>,
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
