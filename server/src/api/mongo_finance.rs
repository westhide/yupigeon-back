use mongo::{
    collection::FinanceVoucherTemplate,
    query::{self, common::QueryTrait},
};
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};
use serde::Deserialize;

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[handler]
pub async fn update_assist_account_items() -> Result<impl IntoResponse> {
    let res = query::finance::assist::update_assist_account_items().await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountInfoParams {
    name: String,
}

#[handler]
pub async fn assist_account_info(
    Query(params): Query<AssistAccountInfoParams>,
) -> Result<impl IntoResponse> {
    let AssistAccountInfoParams { name } = params;

    let res = query::finance::assist::assist_account_info(&name).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeParams {
    code: String,
}

#[handler]
pub async fn assist_account_group_info(
    Query(params): Query<CodeParams>,
) -> Result<impl IntoResponse> {
    let CodeParams { code } = params;

    let res = query::finance::assist::assist_account_group_info(&code).await?;

    Response::json(res)
}

#[handler]
pub async fn finance_account_info(Query(params): Query<CodeParams>) -> Result<impl IntoResponse> {
    let CodeParams { code } = params;

    let res = query::finance::account::finance_account_info(&code).await?;

    Response::json(res)
}

#[handler]
pub async fn insert_finance_voucher_template(
    Json(params): Json<Vec<FinanceVoucherTemplate>>,
) -> Result<impl IntoResponse> {
    let res = FinanceVoucherTemplate::insert_many(params).await?;

    Response::json(res)
}

#[handler]
pub async fn voucher_template_info(Query(params): Query<CodeParams>) -> Result<impl IntoResponse> {
    let CodeParams { code } = params;

    let res = query::finance::voucher::voucher_template_info(&code).await?;

    Response::json(res)
}
