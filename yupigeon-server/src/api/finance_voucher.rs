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
pub async fn voucher_template_info(
    Query(params): Query<VoucherTemplateParams>,
) -> Result<impl IntoResponse> {
    let VoucherTemplateParams { code } = params;

    let subsidiary_account = query::finance_voucher::voucher_template_info(&code)
        .await
        .map_err(BadRequest)?;
    Ok(Json(subsidiary_account))
}
