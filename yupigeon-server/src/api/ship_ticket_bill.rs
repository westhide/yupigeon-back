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
pub struct Params {
    begin_time: String,
    end_time: String,
}

#[handler]
pub async fn get(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params {
        begin_time: begin_str,
        end_time: end_str,
    } = params;

    let begin_time =
        NaiveDateTime::parse_from_str(&begin_str, "%Y-%m-%d %H:%M:%S").map_err(BadRequest)?;
    let end_time =
        NaiveDateTime::parse_from_str(&end_str, "%Y-%m-%d %H:%M:%S").map_err(BadRequest)?;
    let ship_ticket_bill = entity::ship_ticket_bill::get(begin_time, end_time)
        .await
        .map_err(BadRequest)?;
    Ok(Json(ship_ticket_bill))
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
                    if let Err(exec_err) = entity::ship_ticket_bill::refresh().await {
                        println!("refresh failed====>{:?}", exec_err);
                    }
                    global_data.is_ship_ticket_bill_refresh = false;
                });
            }
        }
    }
    Ok(Json(RefreshStatus { is_refresh: true }))
}
