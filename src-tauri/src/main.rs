// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
mod application;
mod domain;
mod infrastructure;
mod interfaces;

use serde::Deserialize;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Debug, Deserialize)]
struct KeyValuePair {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct Request {
    method: String,
    url: String,
    params: Vec<KeyValuePair>,
    headers: Vec<KeyValuePair>,
    body: String,
}

// This function will handle logging messages from the frontend
#[tauri::command]
fn log_message(message: String) {
    println!("{}", message)
}

#[tauri::command]
fn send_request(request: Request) {
    println!("Received request:");
    println!("Method: {}", request.method);
    println!("URL: {}", request.url);
    println!("Query Params:");
    for param in request.params {
        println!("  {}: {}", param.key, param.value);
    }
    println!("Headers:");
    for header in request.headers {
        println!("  {}: {}", header.key, header.value);
    }
    println!("Body: {}", request.body);
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_request, log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
