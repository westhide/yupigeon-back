use database::query;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

#[handler]
pub async fn subsidiary_clients() -> Result<impl IntoResponse> {
    query::finance_subsidiary::subsidiary_clients()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn update_items() -> Result<impl IntoResponse> {
    query::finance_subsidiary::update_items()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct SubsidiaryAccountParams {
    code: String,
}

#[handler]
pub async fn subsidiary_account(
    Query(params): Query<SubsidiaryAccountParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryAccountParams { code } = params;

    query::finance_subsidiary::find_subsidiary_account_by_code(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct SubsidiaryGroupParams {
    id: i32,
}

#[handler]
pub async fn subsidiary_group_info(
    Query(params): Query<SubsidiaryGroupParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryGroupParams { id } = params;

    query::finance_subsidiary::subsidiary_group_info(id)
        .await
        .map_err(BadRequest)
        .map(Json)
}
