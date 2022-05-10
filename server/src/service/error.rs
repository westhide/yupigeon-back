use database::sea_orm::DbErr;
use mongo::error::MongoErr;
use poem::{
    error::{Error as PoemErr, ResponseError},
    http::StatusCode,
    Body, Response,
};

pub type Result<T, E = WrapError> = poem::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum WrapError {
    #[error("{0}")]
    MessageError(String),

    #[error("{0}")]
    DbError(#[from] DbErr),

    #[error("{0}")]
    MongoError(#[from] MongoErr),

    #[error("{0}")]
    PoemError(PoemErr),
}

impl WrapError {
    pub fn message_error(message: &str) -> Self {
        WrapError::MessageError(message.into())
    }
}

impl ResponseError for WrapError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn as_response(&self) -> Response {
        let message = match self {
            Self::MessageError(message) => message.clone(),
            Self::DbError(err) => err.to_string(),
            Self::MongoError(err) => err.to_string(),
            Self::PoemError(err) => err.to_string(),
        };
        let body = Body::from_json(serde_json::json!({
            "errorMessage": message,
        }))
        .unwrap();

        Response::builder().status(self.status()).body(body)
    }
}

impl From<PoemErr> for WrapError {
    fn from(err: PoemErr) -> Self {
        WrapError::PoemError(err)
    }
}
