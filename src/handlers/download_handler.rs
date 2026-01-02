use crate::types::{AppState, DownloadQuery};
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::{Query, State};
use axum::http::Response;
use axum::response::Html;
use std::fmt::format;
use std::sync::Arc;
use std::time::Duration;

#[axum::debug_handler]
pub async fn download_handler(
    State(app_state): State<Arc<AppState>>,
    Query(download_query): Query<DownloadQuery>,
) -> Result<Response<String>, Html<String>> {
    let name = download_query.get_name();
    let s3_client = app_state.s3_client();
    let expires_in = Duration::from_hours(8);
    let presign_config = PresigningConfig::expires_in(expires_in).unwrap();
    let result = s3_client
        .get_object()
        .bucket(app_state.bucket_name())
        .key(name.clone())
        .response_content_disposition(format!("attachment; filename={}", name))
        .response_content_type("application/octet-stream")
        .presigned(presign_config)
        .await;

    match result {
        Ok(object) => {
            let uri = object.uri().to_string();
            let response = Response::builder()
                .status(307)
                .header(
                    "Content-Disposition",
                    format!("attachment; filename={}", name),
                )
                .header("Content-Type", "application/octet-stream")
                .header("Location", uri)
                .body("redirecting...".to_string())
                .unwrap();
            Ok(response)
        }
        Err(msg) => {
            return Err(Html(format!("{:?}", msg)));
        }
    }
}
