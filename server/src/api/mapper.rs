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
    domain: String,
    r#type: String,
}

#[handler]
pub async fn domain_value(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params { domain, r#type } = params;
    query::mapper::domain_value(&domain, &r#type)
        .await
        .map_err(BadRequest)
        .map(Json)
}
