use database::{mongo::error::MongoErr, sea_orm::DbErr, sibyl};
use poem::{
    error::{Error as PoemErr, ResponseError},
    http::StatusCode,
    Body, Response,
};

pub type Result<T, E = WrapError> = poem::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum WrapError {
    #[error("@Service {0}")]
    Message(String),

    #[error("{0}")]
    Db(#[from] DbErr),

    #[error("{0}")]
    Mongo(#[from] MongoErr),

    #[error("{0}")]
    Oracle(#[from] sibyl::Error),

    #[error("{0}")]
    Poem(PoemErr),
}

impl WrapError {
    pub fn message_error(message: &str) -> Self {
        WrapError::Message(message.into())
    }
}

impl ResponseError for WrapError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn as_response(&self) -> Response {
        let message = match self {
            Self::Message(message) => message.into(),
            Self::Db(err) => err.to_string(),
            Self::Mongo(err) => err.to_string(),
            Self::Oracle(err) => err.to_string(),
            Self::Poem(err) => err.to_string(),
        };
        let body = Body::from_json(serde_json::json!({
            "errorMessage": message,
        }));

        let response_builder = Response::builder().status(self.status());
        match body {
            Ok(body) => response_builder.body(body),
            Err(_) => response_builder.body(()),
        }
    }
}

impl From<PoemErr> for WrapError {
    fn from(err: PoemErr) -> Self {
        WrapError::Poem(err)
    }
}
