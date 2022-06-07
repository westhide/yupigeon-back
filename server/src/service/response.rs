use poem::web::Json;
use serde::{Deserialize, Serialize};

use crate::service::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = String> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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

    fn json_response(self) -> Json<Self>
    where
        Self: Sized,
    {
        Json(self)
    }
}

impl Response {
    pub fn message(message: &str) -> Result<Json<Self>> {
        Self::json_with_message(None, message)
    }
}

impl ResponseTrait for Response {}
