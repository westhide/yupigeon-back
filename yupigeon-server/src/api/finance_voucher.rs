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
pub struct VoucherTemplateParams {
    code: String,
}

#[handler]
pub async fn voucher_template(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    query::finance_voucher::voucher_template(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn voucher_template_info(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    query::finance_voucher::voucher_template_info(&code)
        .await
        .map_err(BadRequest)
        .map(Json)
}
