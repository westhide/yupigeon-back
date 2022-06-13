use database::oracle::query;
use poem::{handler, web::Json, IntoResponse};

use crate::service::{
    error::Result,
    params::DateTimeParams,
    response::{Response, ResponseTrait},
};

#[handler]
pub fn ticket_bill(Json(params): Json<DateTimeParams>) -> Result<impl IntoResponse> {
    let DateTimeParams {
        begin_time,
        end_time,
    } = params;

    let res = query::canyon_ticket_bill::ticket_bill(&begin_time, &end_time)?;

    Response::json(res)
}

#[handler]
pub fn ticket_type() -> Result<impl IntoResponse> {
    let res = query::canyon_ticket_type::ticket_type()?;

    Response::json(res)
}
