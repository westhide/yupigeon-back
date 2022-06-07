use chrono::offset::Local;
use database::query;
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};
use serde::Deserialize;

use crate::{
    global_data::{get_websocket_sender, ShipTicketRefreshStatus},
    service::{
        error::Result,
        params::{DateTimeParams, ParseDateTimeParams},
        response::{Response, ResponseTrait},
    },
};

#[handler]
pub async fn bill(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::ship_ticket::bill(begin_time, end_time).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundBillParams {
    #[serde(flatten)]
    datetime_params: DateTimeParams,
    where_condition: Option<String>,
}

#[handler]
pub async fn refund_bill(Json(params): Json<RefundBillParams>) -> Result<impl IntoResponse> {
    let RefundBillParams {
        datetime_params,
        where_condition,
    } = params;

    let (begin_time, end_time) = datetime_params.get_datetime_params()?;

    let where_condition = match where_condition {
        Some(where_condition) => format!(" AND {}", where_condition),
        None => "".into(),
    };

    let res = query::ship_ticket::refund_bill(begin_time, end_time, &where_condition).await?;

    Response::json(res)
}

#[handler]
pub async fn clients() -> Result<impl IntoResponse> {
    let res = query::ship_ticket::clients().await?;

    Response::json(res)
}

#[handler]
pub async fn conductors() -> Result<impl IntoResponse> {
    let res = query::ship_ticket::conductors().await?;

    Response::json(res)
}

#[handler]
pub fn refresh_status() -> Result<impl IntoResponse> {
    let res = ShipTicketRefreshStatus::get()?;

    Response::json(res)
}

#[handler]
pub async fn refresh() -> Result<impl IntoResponse> {
    let ship_ticket_refresh_status = ShipTicketRefreshStatus::get()?;
    if !ship_ticket_refresh_status.is_refresh {
        tokio::spawn(async move {
            ShipTicketRefreshStatus::set(true, None).ok();
            query::ship_ticket::refresh().await.ok();

            ShipTicketRefreshStatus::set(false, Some(Local::now().to_string())).ok();
            if let Ok(sender) = get_websocket_sender("default") {
                sender
                    .send(r#"{"message":"船票明细已更新"}"#.to_string())
                    .ok();
            }
        });
    };

    Response::json(ship_ticket_refresh_status)
}

#[handler]
pub async fn daily_sales(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::ship_ticket::daily_sales(begin_time, end_time).await?;

    Response::json(res)
}

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::ship_ticket::daily_receipt(begin_time, end_time).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSalesParams {
    #[serde(flatten)]
    datetime_params: DateTimeParams,
    where_condition: Option<String>,
}

#[handler]
pub async fn client_sales(Json(params): Json<ClientSalesParams>) -> Result<impl IntoResponse> {
    let ClientSalesParams {
        datetime_params,
        where_condition,
    } = params;

    let (begin_time, end_time) = datetime_params.get_datetime_params()?;

    let where_condition = match where_condition {
        Some(where_condition) => format!(" WHERE {}", where_condition),
        None => "".into(),
    };

    let res = query::ship_ticket::client_sales(begin_time, end_time, &where_condition).await?;

    Response::json(res)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConductorDailyReceiptParams {
    #[serde(flatten)]
    datetime_params: DateTimeParams,
    where_condition: Option<String>,
}

#[handler]
pub async fn conductor_daily_receipt(
    Json(params): Json<ConductorDailyReceiptParams>,
) -> Result<impl IntoResponse> {
    let ConductorDailyReceiptParams {
        datetime_params,
        where_condition,
    } = params;

    let (begin_time, end_time) = datetime_params.get_datetime_params()?;

    let where_condition = match where_condition {
        Some(where_condition) => format!(" AND {}", where_condition),
        None => "".into(),
    };

    let res =
        query::ship_ticket::conductor_daily_receipt(begin_time, end_time, &where_condition).await?;

    Response::json(res)
}

#[handler]
pub async fn ticket_revenue(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::ship_ticket::ticket_revenue(begin_time, end_time).await?;

    Response::json(res)
}

#[handler]
pub async fn fee_revenue(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::ship_ticket::fee_revenue(begin_time, end_time).await?;

    Response::json(res)
}
