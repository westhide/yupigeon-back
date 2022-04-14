use database::query;
use poem::{
    error::BadRequest,
    handler,
    http::StatusCode,
    web::{Json, Query},
    Error, IntoResponse, Result,
};
use serde::{Deserialize, Serialize};

use crate::{
    service::utils::{DateTimeParams, ParseDateTimeParams},
    GLOBAL_DATA,
};

#[handler]
pub async fn bill(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::ship_ticket_bill::bill(begin_time, end_time)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn clients() -> Result<impl IntoResponse> {
    query::ship_ticket_bill::clients()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn conductors() -> Result<impl IntoResponse> {
    query::ship_ticket_bill::conductors()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct RefreshStatus {
    is_refresh: bool,
}

#[handler]
pub async fn refresh_status() -> Result<impl IntoResponse> {
    match GLOBAL_DATA.get() {
        Some(global_data_mutex) => {
            if let Ok(global_data) = global_data_mutex.try_lock() {
                return Ok(Json(RefreshStatus {
                    is_refresh: global_data.is_ship_ticket_bill_refresh,
                }));
            };
            Ok(Json(RefreshStatus { is_refresh: true }))
        }
        None => Err(Error::from_string(
            "Can Not Get GLOBAL_DATA",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[handler]
pub async fn refresh() -> Result<impl IntoResponse> {
    if let Some(global_data_mutex) = GLOBAL_DATA.get() {
        if let Ok(mut global_data) = global_data_mutex.try_lock() {
            if !global_data.is_ship_ticket_bill_refresh {
                tokio::spawn(async move {
                    global_data.is_ship_ticket_bill_refresh = true;
                    query::ship_ticket_bill::refresh().await.ok();
                    global_data.is_ship_ticket_bill_refresh = false;
                });
            }
        }
    }
    Ok(Json(RefreshStatus { is_refresh: true }))
}

#[handler]
pub async fn daily_sales(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::ship_ticket_bill::daily_sales(begin_time, end_time)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::ship_ticket_bill::daily_receipt(begin_time, end_time)
        .await
        .map_err(BadRequest)
        .map(Json)
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
    let where_condition = where_condition.unwrap_or_default();

    query::ship_ticket_bill::client_sales(begin_time, end_time, &where_condition)
        .await
        .map_err(BadRequest)
        .map(Json)
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
    let where_condition = where_condition.unwrap_or_default();

    query::ship_ticket_bill::conductor_daily_receipt(begin_time, end_time, &where_condition)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn voucher_revenue(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::ship_ticket_bill::voucher_revenue(begin_time, end_time)
        .await
        .map_err(BadRequest)
        .map(Json)
}
