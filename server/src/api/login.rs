use database::mysql::query;
use poem::{handler, web::Json, IntoResponse};
use serde::Deserialize;

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    username: String,
    password: String,
}

#[handler]
pub async fn post(Json(params): Json<Params>) -> Result<impl IntoResponse> {
    let Params { username, password } = params;
    let res = query::user::user(username, password).await?;

    Response::json(res)
}
