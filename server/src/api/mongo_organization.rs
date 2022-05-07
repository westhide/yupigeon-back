use mongo::{
    collection::{OrganizationCompany, OrganizationGroup},
    query::common::QueryTrait,
};
use poem::{handler, web::Json, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::MongoError,
};

#[handler]
pub async fn insert_organization_company(
    Json(params): Json<Vec<OrganizationCompany>>,
) -> Result<impl IntoResponse> {
    let res = OrganizationCompany::insert_many(params)
        .await
        .map_err(MongoError)?;

    Response::json(res)
}

#[handler]
pub async fn insert_organization_group(
    Json(params): Json<Vec<OrganizationGroup>>,
) -> Result<impl IntoResponse> {
    let res = OrganizationGroup::insert_many(params)
        .await
        .map_err(MongoError)?;

    Response::json(res)
}
