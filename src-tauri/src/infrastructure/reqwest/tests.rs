use super::*;
use crate::domain::models::request::{Body, HttpRequest, Method, RawBody, RawBodyType};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("Hello, world!")
                .insert_header("content-type", "text/plain"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/test");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
    assert_eq!(response.get_body_as_text().unwrap(), "Hello, world!");
}

#[tokio::test]
async fn test_post_request_with_json() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(serde_json::json!({"message": "Created"}))
                .insert_header("content-type", "application/json"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let body = Body::Raw(RawBody {
        content: r#"{"key": "value"}"#.to_string(),
        content_type: RawBodyType::Json,
    });
    let request = HttpRequest::new(Method::POST, mock_server.uri() + "/test").with_body(body);
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 201);
    assert_eq!(
        response.get_body_as_text().unwrap().as_str(),
        r#"{"message":"Created"}"#
    );
}

#[tokio::test]
async fn test_request_with_custom_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(wiremock::matchers::header("X-Custom-Header", "test-value"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/test")
        .with_header("X-Custom-Header", "test-value");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
}

#[tokio::test]
async fn test_request_with_query_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .and(wiremock::matchers::query_param("key", "value"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request =
        HttpRequest::new(Method::GET, mock_server.uri() + "/test").with_param("key", "value");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
}

#[tokio::test]
async fn test_json_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"key": "value"}))
                .insert_header("content-type", "application/json"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/json");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
    assert!(matches!(response.body, ResponseBody::Json(_)));
    if let ResponseBody::Json(json_value) = response.body {
        assert_eq!(json_value, serde_json::json!({"key": "value"}));
    } else {
        panic!("Expected JSON response body");
    }
}

#[tokio::test]
async fn test_xml_response() {
    let mock_server = MockServer::start().await;

    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <element>
        <name>Test</name>
        <value>123</value>
    </element>
</root>"#;

    Mock::given(method("GET"))
        .and(path("/xml"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(xml_content)
                .insert_header("content-type", "application/xml"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/xml");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
    assert!(matches!(response.body, ResponseBody::Xml(_)));
    if let ResponseBody::Xml(xml_content_response) = response.body {
        assert_eq!(xml_content_response.trim(), xml_content.trim());
    } else {
        panic!("Expected XML response body");
    }
}

#[tokio::test]
async fn test_html_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Hello</body></html>")
                .insert_header("content-type", "text/html"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/html");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
    assert!(matches!(response.body, ResponseBody::Html(_)));
    if let ResponseBody::Html(html_content) = response.body {
        assert_eq!(html_content, "<html><body>Hello</body></html>");
    }
}

#[tokio::test]
async fn test_plain_text_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/text"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("Hello, world!")
                .insert_header("content-type", "text/plain"),
        )
        .mount(&mock_server)
        .await;

    let client = ReqwestClient::new();
    let request = HttpRequest::new(Method::GET, mock_server.uri() + "/text");
    let response = client.send_request(&request).await.unwrap();

    assert_eq!(response.status.code, 200);
    assert!(matches!(response.body, ResponseBody::Text(_)));
    if let ResponseBody::Text(text_content) = response.body {
        assert_eq!(text_content, "Hello, world!");
    } else {
        panic!("Expected Text response body");
    }
}

// TODO: fix timeout issue
// #[tokio::test]
// async fn test_timeout() {
//     let mock_server = MockServer::start().await;
//
//     Mock::given(method("GET"))
//         .and(path("/delay"))
//         .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(2)))
//         .mount(&mock_server)
//         .await;
//
//     let client = ReqwestClient::new();
//     let request = HttpRequest::new(Method::GET, mock_server.uri() + "/delay").with_timeout(1); // 1 second timeout
//
//     let result = client.send_request(&request).await;
//
//     assert!(
//         result.is_err(),
//         "Expected timeout error, but request succeeded"
//     );
// }
