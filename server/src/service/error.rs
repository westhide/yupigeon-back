use poem::{error::ResponseError, http::StatusCode, Body, Response};

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct MyError {
    message: String,
}

impl MyError {
    pub fn new(message: &str) -> MyError {
        Self {
            message: message.into(),
        }
    }
}

impl ResponseError for MyError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_GATEWAY
    }

    fn as_response(&self) -> Response {
        let body = Body::from_json(serde_json::json!({
            "errMessage": self.message,
        }))
        .unwrap();

        Response::builder().status(self.status()).body(body)
    }
}

pub fn error_msg(message: &str) -> MyError {
    MyError::new(message)
}
