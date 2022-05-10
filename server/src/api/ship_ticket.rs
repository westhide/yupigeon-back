use chrono::offset::Local;
use database::query;
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    global_data::get_global_data,
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

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefreshStatus {
    is_refresh: bool,
    last_refresh_datetime: String,
}

#[handler]
pub fn refresh_status() -> Result<impl IntoResponse> {
    let global_data = get_global_data()?;

    let default_datetime = String::from("");
    let last_refresh_datetime = global_data
        .last_refresh_datetime
        .as_ref()
        .unwrap_or(&default_datetime);

    let res = RefreshStatus {
        is_refresh: global_data.is_ship_ticket_bill_refresh,
        last_refresh_datetime: last_refresh_datetime.clone(),
    };

    Response::json(res)
}

#[handler]
pub async fn refresh() -> Result<impl IntoResponse> {
    let mut global_data = get_global_data()?;

    if !global_data.is_ship_ticket_bill_refresh {
        tokio::spawn(async move {
            global_data.is_ship_ticket_bill_refresh = true;

            query::ship_ticket::refresh().await.ok();

            global_data.is_ship_ticket_bill_refresh = false;
            global_data.last_refresh_datetime = Some(Local::now().to_string());
        });
    }

    let res = RefreshStatus {
        is_refresh: true,
        ..Default::default()
    };

    Response::json(res)
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
