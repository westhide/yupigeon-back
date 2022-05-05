use database::sea_orm::DbErr;
use mongo::mongodb::error::Error as MongoErr;
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
        let message = db_err.to_string();
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

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct MongoError(pub MongoErr);

impl ResponseError for MongoError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn as_response(&self) -> Response {
        let mongo_err = &self.0;
        let message = mongo_err.to_string();
        let body = Body::from_json(serde_json::json!({
            "errorMessage": message,
        }))
        .unwrap();

        Response::builder().status(self.status()).body(body)
    }
}

impl From<MongoErr> for MongoError {
    fn from(mongo_err: MongoErr) -> Self {
        MongoError(mongo_err)
    }
}
