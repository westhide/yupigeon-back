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
pub async fn finance_accounts() -> Result<impl IntoResponse> {
    query::finance_account::finance_accounts()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn finance_account_info(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { code } = params;

    query::finance_account::finance_account_info(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}
