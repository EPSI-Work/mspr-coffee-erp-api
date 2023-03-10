use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[cfg(not(tarpaulin_include))]
#[derive(thiserror::Error)]
pub enum APIError {
    #[error("Authentication failed.")]
    AuthorizationError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(not(tarpaulin_include))]
impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match self {
            APIError::UnexpectedError(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            APIError::AuthorizationError(_) => HttpResponse::new(StatusCode::FORBIDDEN),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Debug for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[cfg(not(tarpaulin_include))]
fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
