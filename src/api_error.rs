use std::fmt::Display;

#[derive(Debug)]
pub enum ApiError {
    Error(anyhow::Error),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Error(e)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Error(e) => write!(f, "{}", e),
        }
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ApiError::Error(e) => {
                actix_web::HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}
