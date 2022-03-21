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
