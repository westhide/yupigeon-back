use crate::service::{
    error::Result,
    params::DateTimeParams,
    response::{Response, ResponseTrait},
};
use database::oracle::query;
use poem::{handler, web::Json, IntoResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketBillParams {
    #[serde(flatten)]
    datetime_params: DateTimeParams,
    operators: Vec<String>,
}

#[handler]
pub fn ticket_bill(Json(params): Json<TicketBillParams>) -> Result<impl IntoResponse> {
    let TicketBillParams {
        datetime_params,
        operators,
    } = params;

    let DateTimeParams {
        begin_time,
        end_time,
    } = datetime_params;

    let condition = match operators {
        Some(operators) => format!(" AND so.operatorName IN ({})", operators.join(",")),
        None => "".into(),
    };

    let res = query::canyon_ticket_bill::ticket_bill(&begin_time, &end_time, &condition)?;

    Response::json(res)
}

#[handler]
pub fn ticket_type() -> Result<impl IntoResponse> {
    let res = query::canyon_ticket_type::ticket_type()?;

    Response::json(res)
}

#[handler]
pub fn operators() -> Result<impl IntoResponse> {
    let res = query::operators::operators()?;

    Response::json(res)
}
