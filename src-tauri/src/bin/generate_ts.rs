use pigeon::models::request::*;
use pigeon::models::response::{Body as ResponseBody, HttpResponse, Size, Status, Timing};
use std::{fs::File, io::Write, path::Path};
use ts_rs::TS;

fn main() {
    let mut dst = String::new();

    dst.push_str(&HttpRequest::decl());
    dst.push_str(&Method::decl());
    dst.push_str(&Param::decl());
    dst.push_str(&Body::decl());
    dst.push_str(&RawBody::decl());
    dst.push_str(&RawBodyType::decl());
    dst.push_str(&FormDataItem::decl());
    dst.push_str(&FormDataValue::decl());
    dst.push_str(&HttpResponse::decl());
    dst.push_str(&ResponseBody::decl());
    dst.push_str(&Size::decl());
    dst.push_str(&Status::decl());
    dst.push_str(&Timing::decl());

    let file_path = "../src/types/generated.ts";
    let path = Path::new(file_path);

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }

    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(path).expect("Failed to create file");

    // Write the TypeScript definitions to the file
    file.write_all(dst.as_bytes())
        .expect("Failed to write to file");

    println!("TypeScript definitions written to {}", file_path);
}
