use database::mongo::{
    collection::{OrganizationCompany, OrganizationGroup},
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
pub async fn insert_organization_company(
    Json(params): Json<Vec<OrganizationCompany>>,
) -> Result<impl IntoResponse> {
    let res = OrganizationCompany::insert_many(params).await?;

    Response::json(res)
}

#[handler]
pub async fn insert_organization_group(
    Json(params): Json<Vec<OrganizationGroup>>,
) -> Result<impl IntoResponse> {
    let res = OrganizationGroup::insert_many(params).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeParams {
    finance_code: String,
}

#[handler]
pub async fn organization_company(Query(params): Query<CodeParams>) -> Result<impl IntoResponse> {
    let CodeParams { finance_code } = params;

    let res = query::organization::organization_company(&finance_code).await?;

    Response::json(res)
}
