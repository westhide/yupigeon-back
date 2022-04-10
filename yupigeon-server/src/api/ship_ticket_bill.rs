use chrono::NaiveDateTime;
use database::entity;
use poem::{
    error::BadRequest,
    handler,
    http::StatusCode,
    web::{Json, Query},
    Error, IntoResponse, Result,
};
use serde::{Deserialize, Serialize};

use crate::GLOBAL_DATA;

#[derive(Debug, Deserialize)]
pub struct DateTimeParams {
    begin_time: String,
    end_time: String,
}

fn parse_datetime(time_str: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").map_err(BadRequest)
}

#[handler]
pub async fn bill(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let DateTimeParams {
        begin_time: begin_time_str,
        end_time: end_time_str,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let ship_ticket_bill = entity::ship_ticket_bill::ship_ticket_bill(begin_time, end_time)
        .await
        .map_err(BadRequest)?;
    Ok(Json(ship_ticket_bill))
}

#[handler]
pub async fn clients() -> Result<impl IntoResponse> {
    let clients = database::ship_ticket_bill::clients()
        .await
        .map_err(BadRequest)?;
    Ok(Json(clients))
}

#[handler]
pub async fn conductors() -> Result<impl IntoResponse> {
    let conductors = database::ship_ticket_bill::conductors()
        .await
        .map_err(BadRequest)?;
    Ok(Json(conductors))
}

#[derive(Debug, Deserialize, Serialize, Default)]
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
                    database::ship_ticket_bill::refresh().await.unwrap();
                    global_data.is_ship_ticket_bill_refresh = false;
                });
            }
        }
    }
    Ok(Json(RefreshStatus { is_refresh: true }))
}

#[handler]
pub async fn daily_sales(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let DateTimeParams {
        begin_time: begin_time_str,
        end_time: end_time_str,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let daily_sales = database::ship_ticket_bill::daily_sales(begin_time, end_time)
        .await
        .map_err(BadRequest)?;
    Ok(Json(daily_sales))
}

#[handler]
pub async fn daily_receipt(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let DateTimeParams {
        begin_time: begin_time_str,
        end_time: end_time_str,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let daily_receipt = database::ship_ticket_bill::daily_receipt(begin_time, end_time)
        .await
        .map_err(BadRequest)?;
    Ok(Json(daily_receipt))
}

#[derive(Debug, Deserialize)]
pub struct ClientSalesParams {
    begin_time: String,
    end_time: String,
    where_condition: Option<String>,
}

#[handler]
pub async fn client_sales(Json(params): Json<ClientSalesParams>) -> Result<impl IntoResponse> {
    let ClientSalesParams {
        begin_time: begin_time_str,
        end_time: end_time_str,
        where_condition,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let where_condition = where_condition.unwrap_or_default();
    let client_sales =
        database::ship_ticket_bill::client_sales(begin_time, end_time, &where_condition)
            .await
            .map_err(BadRequest)?;
    Ok(Json(client_sales))
}

#[derive(Debug, Deserialize)]
pub struct ConductorDailyReceiptParams {
    begin_time: String,
    end_time: String,
    where_condition: Option<String>,
}

#[handler]
pub async fn conductor_daily_receipt(
    Json(params): Json<ConductorDailyReceiptParams>,
) -> Result<impl IntoResponse> {
    let ConductorDailyReceiptParams {
        begin_time: begin_time_str,
        end_time: end_time_str,
        where_condition,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let where_condition = where_condition.unwrap_or_default();
    let conductor_daily_receipt =
        database::ship_ticket_bill::conductor_daily_receipt(begin_time, end_time, &where_condition)
            .await
            .map_err(BadRequest)?;
    Ok(Json(conductor_daily_receipt))
}
