use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(thiserror::Error)]
pub enum APIError {
    #[error("Authentication failed.")]
    AuthorizationError(#[source] anyhow::Error, String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match self {
            APIError::UnexpectedError(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            APIError::AuthorizationError(_, error_message) => HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body(json!({"message": error_message.to_owned()}).to_string()),
        }
    }
}

impl std::fmt::Debug for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::http::header::CONTENT_TYPE;

    #[actix_rt::test]
    async fn test_unexpected_error() {
        let error = APIError::UnexpectedError(anyhow::anyhow!("Unexpected error"));

        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body, "");
    }

    #[actix_rt::test]
    async fn test_authorization_error() {
        let error = APIError::AuthorizationError(
            anyhow::anyhow!("Authorization failed"),
            "Invalid token".to_owned(),
        );

        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/json"
        );

        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body, r#"{"message":"Invalid token"}"#);
    }

    #[test]
    fn test_debug_impl() {
        let error = APIError::AuthorizationError(
            anyhow::anyhow!("Authorization failed"),
            "Invalid token".to_owned(),
        );

        let output = format!("{:#?}", error);

        assert!(output.contains("Authorization failed"));
        assert!(output.contains("Caused by:"));
    }

    #[test]
    fn test_unexpected_error_display() {
        let error = APIError::UnexpectedError(anyhow::anyhow!("Unexpected error"));
        assert!(error.to_string().contains("Unexpected error"));
    }

    #[test]
    fn test_authorization_error_display() {
        let error = APIError::AuthorizationError(
            anyhow::anyhow!("Authorization failed"),
            "Invalid token".to_owned(),
        );
        assert!(error.to_string().contains("Authentication failed."));
    }
}
