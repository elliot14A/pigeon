use crate::domain::{
    error::Result,
    models::{request::HttpRequest, response::HttpResponse},
};
use async_trait::async_trait;

#[async_trait]
pub trait HttpClientRepository: Send + Sync + 'static {
    async fn send_request(&self, req: &HttpRequest) -> Result<HttpResponse>;
}
