use askama::Template;
use aws_sdk_s3::operation::head_object::HeadObjectOutput;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FileQuery {
    name: String,
}

impl FileQuery {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Template)]
#[template(path = "meta.html")]
#[derive(Serialize, Deserialize, Debug)]
pub struct FileMeta {
    pub file_name: String,
    pub file_type: Option<String>,
    pub size: Option<String>,
    pub download_link: String,
}

impl FileMeta {
    pub fn new(object: HeadObjectOutput, file_name: String) -> Self {
        let file_type = object.content_type().map(|t| t.to_string());
        let size = object
            .content_length()
            .map(|val| format!("{:.2} MB", val as f64 / 1024.0 / 1024.0));

        if let Some(metadata) = object.metadata() {
            for (key, value) in metadata.iter() {
                println!("{}: {}", key, value);
            }
        }

        Self {
            file_name: file_name.clone(),
            file_type,
            size,
            download_link: format!("/download?name={}", file_name),
        }
    }
}

impl IntoResponse for FileMeta {
    fn into_response(self) -> Response {
        let body = serde_json::json!(self).to_string();

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}