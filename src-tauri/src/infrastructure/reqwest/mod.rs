#[cfg(test)]
mod tests;

use crate::application::ports::HttpClientRepository;
use crate::domain::error::Result;
use crate::domain::models::request::{
    Body, FormDataValue, HttpRequest, Method, Param, RawBodyType,
};
use crate::domain::models::response::{Body as ResponseBody, HttpResponse, Status, Timing};
use async_trait::async_trait;
use reqwest::{header, Client, RequestBuilder};
use std::collections::HashMap;
use tokio::time::Instant;

pub struct ReqwestClient {
    client: Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // Helper function to convert our domain RequestMethod to reqwest::Method
    fn to_reqwest_method(method: &Method) -> reqwest::Method {
        match method {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
            Method::PATCH => reqwest::Method::PATCH,
            Method::HEAD => reqwest::Method::HEAD,
            Method::OPTIONS => reqwest::Method::OPTIONS,
        }
    }

    // Helper function to build headers
    fn build_headers(&self, headers: &HashMap<String, String>) -> header::HeaderMap {
        let mut header_map = header::HeaderMap::new();
        for (key, value) in headers {
            if let (Ok(name), Ok(val)) = (
                header::HeaderName::from_bytes(key.as_bytes()),
                header::HeaderValue::from_str(value),
            ) {
                header_map.insert(name, val);
            }
        }
        header_map
    }

    fn add_query_params(&self, builder: RequestBuilder, params: &[Param]) -> RequestBuilder {
        builder.query(
            &params
                .iter()
                .map(|p| (&p.key, &p.value))
                .collect::<Vec<_>>(),
        )
    }

    fn add_body(&self, builder: RequestBuilder, body: &Option<Body>) -> RequestBuilder {
        match body {
            Some(Body::Raw(raw_body)) => {
                let content_type = match raw_body.content_type {
                    RawBodyType::Text => "text/plain",
                    RawBodyType::Json => "application/json",
                    RawBodyType::Xml => "application/xml",
                    RawBodyType::Html => "text/html",
                    RawBodyType::Javascript => "application/Javascript",
                };
                builder
                    .header(header::CONTENT_TYPE, content_type)
                    .body(raw_body.content.clone())
            }
            Some(Body::FormData(form_data)) => {
                let mut form = reqwest::multipart::Form::new();
                for item in form_data {
                    match &item.value {
                        FormDataValue::Text(text) => {
                            form = form.text(item.key.clone(), text.clone())
                        }
                        FormDataValue::File { path, content_type } => {
                            if let Ok(file_content) = std::fs::read(path) {
                                let file_name = std::path::Path::new(path)
                                    .file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or("file")
                                    .to_string();
                                let mut part =
                                    reqwest::multipart::Part::stream(file_content.clone())
                                        .file_name(file_name.to_string());
                                if let Some(ct) = content_type {
                                    if let Ok(mime) = ct.parse::<mime::Mime>() {
                                        part = part.mime_str(mime.as_ref()).unwrap_or_else(|_| {
                                            reqwest::multipart::Part::stream(file_content)
                                        });
                                    }
                                }
                                form = form.part(item.key.clone(), part);
                            }
                        }
                    }
                }
                builder.multipart(form)
            }
            Some(Body::UrlEncoded(params)) => builder.form(
                &params
                    .iter()
                    .map(|p| (&p.key, &p.value))
                    .collect::<Vec<_>>(),
            ),
            Some(Body::Binary(path)) => {
                if let Ok(bytes) = std::fs::read(path) {
                    builder.body(bytes)
                } else {
                    builder
                }
            }
            None => builder,
        }
    }

    // Helper function to handle the response
    async fn handle_response(
        &self,
        response: reqwest::Response,
        start_time: Instant,
    ) -> Result<HttpResponse> {
        let status = Status {
            code: response.status().as_u16(),
            text: response.status().to_string(),
        };

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let content_type = headers
            .get("content-type")
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let body = if content_type.contains("application/json") {
            ResponseBody::Json(response.json().await.unwrap_or_default())
        } else if content_type.contains("text/html") {
            ResponseBody::Html(response.text().await.unwrap_or_default())
        } else if content_type.contains("application/xml") || content_type.contains("text/xml") {
            ResponseBody::Xml(response.text().await.unwrap_or_default())
        } else if content_type.starts_with("text/") {
            ResponseBody::Text(response.text().await.unwrap_or_default())
        } else {
            ResponseBody::Binary(response.bytes().await.unwrap_or_default().to_vec())
        };

        let endtime = &Instant::now();
        let duration = endtime.duration_since(start_time);

        let timing = Timing {
            start: start_time.elapsed().as_secs_f64(),
            end: endtime.elapsed().as_secs_f64(),
            duration: duration.as_secs_f64(),
        };

        let size = crate::domain::models::response::Size {
            headers: headers.iter().map(|(k, v)| k.len() + v.len()).sum(),
            body: match &body {
                ResponseBody::Text(s) | ResponseBody::Html(s) | ResponseBody::Xml(s) => s.len(),
                ResponseBody::Json(j) => j.to_string().len(),
                ResponseBody::Binary(b) => b.len(),
                ResponseBody::Empty => 0,
            },
        };

        Ok(HttpResponse {
            status,
            headers,
            body,
            timing,
            size,
        })
    }
}

#[async_trait]
impl HttpClientRepository for ReqwestClient {
    async fn send_request(&self, request: &HttpRequest) -> Result<HttpResponse> {
        let start_time = Instant::now();
        let method = Self::to_reqwest_method(&request.method);
        let headers = self.build_headers(&request.headers);

        let mut builder = self.client.request(method, &request.url).headers(headers);

        builder = self.add_query_params(builder, &request.params);
        builder = self.add_body(builder, &request.body);

        if let Some(timeout) = request.timeout {
            builder = builder.timeout(std::time::Duration::from_secs(timeout))
        }

        let response = builder.send().await.unwrap();

        self.handle_response(response, start_time).await
    }
}
