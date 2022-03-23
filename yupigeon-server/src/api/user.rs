use database::entity;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    username: String,
    password: String,
}

#[handler]
pub async fn get(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { username, password } = params;
    let user = entity::user::get(username, password)
        .await
        .map_err(BadRequest)?;
    Ok(Json(user))
}
