use database::mysql::query;
use poem::{handler, web::Query, IntoResponse};
use serde::Deserialize;

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    code: String,
}

#[handler]
pub async fn finance_accounts() -> Result<impl IntoResponse> {
    let res = query::finance::finance_accounts().await?;

    Response::json(res)
}

#[handler]
pub async fn finance_account_info(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { code } = params;

    let res = query::finance::finance_account_info(&code).await?;

    Response::json(res)
}

#[handler]
pub async fn subsidiary_clients() -> Result<impl IntoResponse> {
    let res = query::finance::subsidiary_clients().await?;

    Response::json(res)
}

#[handler]
pub async fn update_subsidiary_account_items() -> Result<impl IntoResponse> {
    let res = query::finance::update_subsidiary_account_items().await?;

    Response::json(res)
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

    let res = query::finance::find_subsidiary_account_by_code(&code).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryGroupParams {
    id: u32,
}

#[handler]
pub async fn subsidiary_group_info(
    Query(params): Query<SubsidiaryGroupParams>,
) -> Result<impl IntoResponse> {
    let SubsidiaryGroupParams { id } = params;

    let res = query::finance::subsidiary_group_info(id).await?;

    Response::json(res)
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

    let res = query::finance::voucher_template(&code).await?;

    Response::json(res)
}

#[handler]
pub async fn voucher_template_info(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    let res = query::finance::voucher_template_info(&code).await?;

    Response::json(res)
}

#[handler]
pub async fn voucher_template_group(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    let res = query::finance::voucher_template_group(&code).await?;

    Response::json(res)
}
