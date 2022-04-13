use database::query;
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
    query::user::user(username, password)
        .await
        .map_err(BadRequest)
        .map(Json)
}
