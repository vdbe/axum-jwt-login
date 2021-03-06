use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Argon2(#[from] argon2::Error),
    #[error(transparent)]
    AxumExtension(#[from] axum::extract::rejection::ExtensionRejection),
    #[error(transparent)]
    AxumTypedHeader(#[from] axum::extract::rejection::TypedHeaderRejection),
    #[error(transparent)]
    DieselResult(#[from] diesel::result::Error),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    R2d2(#[from] r2d2::Error),
    #[error(transparent)]
    TokioRecv(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("email is already taken")]
    DuplicateUserEmail,
    #[error("name is already taken")]
    DuplicateUserName,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::Jwt(ref err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::InvalidSignature => StatusCode::UNAUTHORIZED,
                //jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => todo!(),
                //jsonwebtoken::errors::ErrorKind::InvalidRsaKey(_) => todo!(),
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(_) => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => StatusCode::UNAUTHORIZED,
                jsonwebtoken::errors::ErrorKind::InvalidAudience => StatusCode::UNAUTHORIZED,
                jsonwebtoken::errors::ErrorKind::InvalidSubject => StatusCode::UNAUTHORIZED,
                jsonwebtoken::errors::ErrorKind::ImmatureSignature => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => StatusCode::BAD_REQUEST,
                jsonwebtoken::errors::ErrorKind::MissingAlgorithm => StatusCode::BAD_REQUEST,
                //jsonwebtoken::errors::ErrorKind::Base64(_) => todo!(),
                //jsonwebtoken::errors::ErrorKind::Json(_) => todo!(),
                //jsonwebtoken::errors::ErrorKind::Utf8(_) => todo!(),
                //jsonwebtoken::errors::ErrorKind::Crypto(_) => todo!(),
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
