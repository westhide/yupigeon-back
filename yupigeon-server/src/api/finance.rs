use database::query;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    code: String,
}

#[handler]
pub async fn finance_accounts() -> Result<impl IntoResponse> {
    query::finance::finance_accounts()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn finance_account_info(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { code } = params;

    query::finance::finance_account_info(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn subsidiary_clients() -> Result<impl IntoResponse> {
    query::finance::subsidiary_clients()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn update_items() -> Result<impl IntoResponse> {
    query::finance::update_subsidiary_account_items()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryAccountParams {
    code: String,
}

#[handler]
pub async fn subsidiary_account(
    Query(params): Query<SubsidiaryAccountParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryAccountParams { code } = params;

    query::finance::find_subsidiary_account_by_code(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryGroupParams {
    id: i32,
}

#[handler]
pub async fn subsidiary_group_info(
    Query(params): Query<SubsidiaryGroupParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryGroupParams { id } = params;

    query::finance::subsidiary_group_info(id)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTemplateParams {
    code: String,
}

#[handler]
pub async fn voucher_template(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    query::finance::voucher_template(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn voucher_template_info(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    query::finance::voucher_template_info(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn voucher_template_group(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    query::finance::voucher_template_group(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}
