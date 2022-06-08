use database::mysql::query;
use poem::{handler, IntoResponse};

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[handler]
pub async fn bill() -> Result<impl IntoResponse> {
    let res = query::commercial_street::bill().await?;

    Response::json(res)
}

#[handler]
pub async fn rent_revenue() -> Result<impl IntoResponse> {
    let res = query::commercial_street::rent_revenue().await?;

    Response::json(res)
}
