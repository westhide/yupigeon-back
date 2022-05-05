use database::query;
use poem::{handler, web::Query, IntoResponse, Result};

use crate::service::{
    common::{Response, ResponseTrait},
    error::DbError,
    utils::{DateTimeParams, ParseDateTimeParams},
};

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::tenpay::daily_receipt(begin_time, end_time)
        .await
        .map_err(DbError)?;

    Response::json(res)
}
