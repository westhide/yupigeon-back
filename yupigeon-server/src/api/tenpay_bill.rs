use chrono::NaiveDateTime;
use database::entity;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    begin_time: String,
    end_time: String,
}

fn parse_datetime(time_str: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").map_err(BadRequest)
}

#[handler]
pub async fn daily_receipt(Query(params): Query<Params>) -> Result<impl IntoResponse> {
    let Params {
        begin_time: begin_time_str,
        end_time: end_time_str,
    } = params;

    let begin_time = parse_datetime(&begin_time_str)?;
    let end_time = parse_datetime(&end_time_str)?;
    let daily_receipt = entity::tenpay_bill::daily_receipt(begin_time, end_time)
        .await
        .map_err(BadRequest)?;
    Ok(Json(daily_receipt))
}
