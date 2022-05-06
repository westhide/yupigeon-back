use mongo::query;
use poem::{handler, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::MongoError,
};

#[handler]
pub async fn collection_names() -> Result<impl IntoResponse> {
    let res = query::database_info::collection_names()
        .await
        .map_err(MongoError)?;

    Response::json(res)
}
