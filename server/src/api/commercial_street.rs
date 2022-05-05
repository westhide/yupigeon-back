use database::query;
use poem::{error::BadRequest, handler, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::DbError,
};

#[handler]
pub async fn bill() -> Result<impl IntoResponse> {
    let res = query::commercial_street::bill().await.map_err(DbError)?;

    Response::json(res)
}

#[handler]
pub async fn rent_revenue() -> Result<impl IntoResponse> {
    let res = query::commercial_street::rent_revenue()
        .await
        .map_err(BadRequest)?;

    Response::json(res)
}
