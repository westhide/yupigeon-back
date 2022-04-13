use database::query;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};

use crate::service::utils::{DateTimeParams, ParseDateTimeParams};

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::tenpay_bill::daily_receipt(begin_time, end_time)
        .await
        .map_err(BadRequest)
        .map(Json)
}
