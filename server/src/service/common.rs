use poem::{web::Json, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = String> {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub trait ResponseTrait {
    fn json_with_message<T>(result: Option<T>, message: &str) -> Result<Json<Response<T>>> {
        Ok(Json(Response {
            result,
            message: Some(message.into()),
        }))
    }

    fn json<T>(result: T) -> Result<Json<Response<T>>> {
        Ok(Json(Response {
            result: Some(result),
            message: None,
        }))
    }
}

impl Response {
    pub fn message(message: &str) -> Result<Json<Self>> {
        Ok(Json(Response {
            result: None,
            message: Some(message.into()),
        }))
    }
}

impl ResponseTrait for Response {}
