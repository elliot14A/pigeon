use crate::application::ports::HttpClientRepository;
use crate::domain::error::Result;
use crate::domain::models::request::HttpRequest;
use crate::domain::models::response::HttpResponse;

pub struct HttpClientService<R>
where
    R: HttpClientRepository,
{
    client: R,
}

impl<R> HttpClientService<R>
where
    R: HttpClientRepository,
{
    pub fn new(client: R) -> Self {
        Self { client }
    }

    pub async fn send_request(&self, req: &HttpRequest) -> Result<HttpResponse> {
        self.client.send_request(&req).await
    }
}
