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
    let subsidiary_clients = query::finance_subsidiary::subsidiary_clients()
        .await
        .map_err(BadRequest)?;
    Ok(Json(subsidiary_clients))
}

#[handler]
pub async fn update_items() -> Result<impl IntoResponse> {
    let result = query::finance_subsidiary::update_items()
        .await
        .map_err(BadRequest)?;

    Ok(Json(result))
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

    let subsidiary_account = query::finance_subsidiary::find_subsidiary_account_by_code(&code)
        .await
        .map_err(BadRequest)?;
    Ok(Json(subsidiary_account))
}

#[derive(Debug, Deserialize)]
pub struct SubsidiaryGroupParams {
    id: i32,
}

#[handler]
pub async fn subsidiary_group(
    Query(params): Query<SubsidiaryGroupParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryGroupParams { id } = params;
    let subsidiary_group = query::finance_subsidiary::subsidiary_group(id)
        .await
        .map_err(BadRequest)?;
    Ok(Json(subsidiary_group))
}
