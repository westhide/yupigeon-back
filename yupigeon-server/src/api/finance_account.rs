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
    code: String,
}

#[handler]
pub async fn get(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { code } = params;

    let finance_account = query::finance_account::finance_account(&code)
        .await
        .map_err(BadRequest)?;
    Ok(Json(finance_account))
}
