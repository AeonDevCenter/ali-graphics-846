use thiserror::Error;

#[derive(Debug, Error)]
enum HttpError {
    #[error("File Not Found")]
    FileNotFound,
    

}
