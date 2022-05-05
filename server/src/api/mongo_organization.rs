use mongo::{collection::OrganizationCompany, query};
use poem::{handler, web::Json, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::MongoError,
};

#[handler]
pub async fn insert_organization_company(
    Json(params): Json<Vec<OrganizationCompany>>,
) -> Result<impl IntoResponse> {
    let res = query::common::insert_many(params, "OrganizationCompany")
        .await
        .map_err(MongoError)?;

    Response::json(res)
}
