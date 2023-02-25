use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[cfg(not(tarpaulin_include))]
#[derive(thiserror::Error)]
pub enum APIError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(not(tarpaulin_include))]
impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            APIError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
