use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResponse {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Body,
    pub timing: Timing,
    pub size: Size,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Status {
    pub code: u16,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub enum Body {
    Empty,
    Text(String),
    Json(serde_json::Value),
    Html(String),
    Xml(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Timing {
    pub start: f64,
    pub end: f64,
    pub duration: f64,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Size {
    pub headers: usize,
    pub body: usize,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: String) -> Self {
        HttpResponse {
            status: Status {
                code: status_code,
                text: status_text,
            },
            headers: HashMap::new(),
            body: Body::Empty,
            timing: Timing {
                start: 0.0,
                end: 0.0,
                duration: 0.0,
            },
            size: Size {
                headers: 0,
                body: 0,
            },
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    pub fn with_timing(mut self, start: f64, end: f64) -> Self {
        self.timing = Timing {
            start,
            end,
            duration: end - start,
        };
        self
    }

    pub fn get_content_type(&self) -> Option<&str> {
        self.headers.get("Content-Type").map(|s| s.as_str())
    }

    pub fn get_body_as_text(&self) -> Option<String> {
        match &self.body {
            Body::Text(s) | Body::Html(s) | Body::Xml(s) => Some(s.to_owned()),
            Body::Json(v) => Some(v.to_string()),
            _ => None,
        }
    }
}
