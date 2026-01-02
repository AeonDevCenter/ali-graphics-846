use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub title: String,
    pub message: String,
}

impl ErrorPage {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
        }
    }

    pub fn render_html(&self) -> axum::response::Html<String> {
        axum::response::Html(
            self.render()
                .unwrap_or_else(|_| "<h1>Unknown error</h1>".to_string()),
        )
    }
}
