use database::query;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoucherTemplateParams {
    code: String,
}

#[handler]
pub async fn voucher_template(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    let voucher_template = query::finance_voucher::voucher_template(&code)
        .await
        .map_err(BadRequest)?;
    Ok(Json(voucher_template))
}

#[handler]
pub async fn voucher_template_info(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    let voucher_template_info = query::finance_voucher::voucher_template_info(&code)
        .await
        .map_err(BadRequest)?;
    Ok(Json(voucher_template_info))
}
