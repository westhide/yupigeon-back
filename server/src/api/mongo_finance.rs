use mongo::query;
use poem::{handler, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::MongoError,
};

#[handler]
pub async fn update_assist_account_items() -> Result<impl IntoResponse> {
    let res = query::finance::assist::update_assist_account_items()
        .await
        .map_err(MongoError)?;

    Response::json(res)
}

#[handler]
pub async fn assist_account() -> Result<impl IntoResponse> {
    let res = query::finance::assist::assist_account()
        .await
        .map_err(MongoError)?;

    Response::json(res)
}
