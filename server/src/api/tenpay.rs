use database::mysql::query;
use poem::{handler, web::Query, IntoResponse};

use crate::service::{
    error::Result,
    params::{DateTimeParams, ParseDateTimeParams},
    response::{Response, ResponseTrait},
};

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::tenpay::daily_receipt(begin_time, end_time).await?;

    Response::json(res)
}
