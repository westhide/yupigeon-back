use database::query;
use poem::{handler, web::Query, IntoResponse, Result};
use serde::Deserialize;

use crate::service::{
    common::{Response, ResponseTrait},
    error::DbError,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    username: String,
    password: String,
}

#[handler]
pub async fn get(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { username, password } = params;
    let res = query::user::user(username, password)
        .await
        .map_err(DbError)?;

    Response::json(res)
}
