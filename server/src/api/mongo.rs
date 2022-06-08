use database::mongo::query;
use poem::{handler, IntoResponse};

use crate::service::{
    error::Result,
    response::{Response, ResponseTrait},
};

#[handler]
pub async fn collection_names() -> Result<impl IntoResponse> {
    let res = query::database_info::collection_names().await?;

    Response::json(res)
}
