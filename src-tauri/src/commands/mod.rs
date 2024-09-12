use tauri::command;

use crate::error::Result;
use crate::models::response::HttpResponse;
use crate::{models::request::HttpRequest, reqwest::ReqwestClient};

#[command]
pub async fn send_request(request: HttpRequest) -> Result<HttpResponse> {
    let client = ReqwestClient::new();
    client.send_request(&request).await
}
