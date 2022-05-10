use database::query;
use poem::{handler, web::Query, IntoResponse};
use serde::Deserialize;

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    domain: String,
    r#type: String,
}

#[handler]
pub async fn domain_value(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { domain, r#type } = params;
    let res = query::mapper::domain_value(&domain, &r#type).await?;

    Response::json(res)
}
