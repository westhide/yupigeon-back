use poem::web::Json;
use serde::{Deserialize, Serialize};

use crate::service::error::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = String> {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub trait ResponseTrait {
    fn json_with_message<T, E>(result: Option<T>, message: &str) -> Result<Json<Response<T>>, E> {
        Ok(Json(Response {
            result,
            message: Some(message.into()),
        }))
    }

    fn json<T, E>(result: T) -> Result<Json<Response<T>>, E> {
        Ok(Json(Response {
            result: Some(result),
            message: None,
        }))
    }

    fn json_response<E>(self) -> Result<Json<Response<Self>>, E>
    where
        Self: Sized,
    {
        Ok(Json(Response {
            result: Some(self),
            message: None,
        }))
    }
}

impl Response {
    pub fn message(message: &str) -> Result<Json<Self>> {
        Self::json_with_message(None, message)
    }
}

impl ResponseTrait for Response {}
