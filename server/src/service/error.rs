use database::sea_orm::DbErr;
use poem::{error::ResponseError, http::StatusCode, Body, Response};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct MessageError(String);

impl MessageError {
    pub fn new(message: &str) -> Self {
        Self(message.into())
    }
}

impl ResponseError for MessageError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn as_response(&self) -> Response {
        let body = Body::from_json(serde_json::json!({
            "errorMessage": self.0,
        }))
        .unwrap();

        Response::builder().status(self.status()).body(body)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct DbError(pub DbErr);

impl ResponseError for DbError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn as_response(&self) -> Response {
        let db_err = &self.0;
        let message = match db_err {
            DbErr::Conn(message) => message,
            DbErr::Exec(message) => message,
            DbErr::Query(message) => message,
            DbErr::RecordNotFound(message) => message,
            DbErr::Custom(message) => message,
            DbErr::Type(message) => message,
            DbErr::Json(message) => message,
        };
        let body = Body::from_json(serde_json::json!({
            "errorMessage": message,
        }))
        .unwrap();

        Response::builder().status(self.status()).body(body)
    }
}

impl From<DbErr> for DbError {
    fn from(db_err: DbErr) -> Self {
        DbError(db_err)
    }
}
