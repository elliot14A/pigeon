use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub params: Vec<Param>,
    pub body: Option<Body>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Body {
    Raw(RawBody),
    FormData(Vec<FormDataItem>),
    UrlEncoded(Vec<Param>),
    Binary(String), // File path
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RawBody {
    pub content: String,
    pub content_type: RawBodyType,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RawBodyType {
    Text,
    Json,
    Xml,
    Html,
    Javascript,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FormDataItem {
    pub key: String,
    pub value: FormDataValue,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum FormDataValue {
    Text(String),
    File {
        path: String,
        content_type: Option<String>,
    },
}

impl HttpRequest {
    pub fn new(method: Method, url: String) -> Self {
        HttpRequest {
            method,
            url,
            headers: HashMap::new(),
            params: Vec::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.push(Param {
            key: key.into(),
            value: value.into(),
        });
        self
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
