use database::query;
use poem::{error::BadRequest, handler, web::Json, IntoResponse, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    username: String,
    password: String,
}

#[handler]
pub async fn post(Json(params): Json<Params>) -> Result<impl IntoResponse> {
    let Params { username, password } = params;
    query::user::user(username, password)
        .await
        .map_err(BadRequest)
        .map(Json)
}
