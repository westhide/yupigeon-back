use database::query;
use poem::{error::BadRequest, handler, web::Json, IntoResponse, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    username: String,
    password: String,
}

#[handler]
pub async fn post(Json(params): Json<Params>) -> Result<impl IntoResponse> {
    let Params { username, password } = params;
    let user = query::user::user(username, password)
        .await
        .map_err(BadRequest)?;
    Ok(Json(user))
}
