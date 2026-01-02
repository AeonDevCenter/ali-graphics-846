use std::io::ErrorKind;
use crate::types::{AppState, FileMeta, FileQuery};
use askama::Template;
use axum::{
    extract::{Query, State},
};
use std::sync::Arc;
use crate::types::ErrorPage;

#[axum::debug_handler]
pub async fn aws_file_handler(
    State(app_state): State<Arc<AppState>>,
    Query(file_query): Query<FileQuery>,
) -> Result<axum::response::Html<String>, axum::response::Html<String>> {
    let s3 = app_state.s3_client();
    let result = s3
        .head_object()
        .bucket(app_state.bucket_name())
        .key(file_query.get_name())
        .send()
        .await;

    match result {
        Ok(head_object) => {
            let template = FileMeta::new(head_object, file_query.get_name().to_string());
            template.render().map(axum::response::Html).map_err(|e| {
                ErrorPage::new(
                    "Rendering Error",
                    format!("Something went wrong rendering the page: {}", e),
                )
                .render_html()
            })
        }
        Err(err) => {
            Err(ErrorPage::new(
                "File Not Found",
                format!(
                    "We couldn't find the file `{}`. Maybe it was removed or the name is incorrect.",
                    file_query.get_name()
                ),
            )
                .render_html())
        }
    }
}
