use chrono::NaiveDateTime;
use database::entity;
use poem::{
    handler,
    web::{Json, Query},
};
use serde::Deserialize;
#[derive(Debug, Deserialize)]

pub struct Params {
    begin_time: String,
    end_time: String,
}

#[handler]
pub async fn get(Query(params): Query<Params>) -> Json<Vec<entity::ship_ticket_bill::Model>> {
    let Params {
        begin_time: begin_str,
        end_time: end_str,
    } = params;
    let begin_time = NaiveDateTime::parse_from_str(&begin_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let end_time = NaiveDateTime::parse_from_str(&end_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let ship_ticket_bill = entity::ship_ticket_bill::get(begin_time, end_time).await;
    Json(ship_ticket_bill)
}
