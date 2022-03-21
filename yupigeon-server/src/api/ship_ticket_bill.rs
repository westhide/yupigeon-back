use database::entity;
use poem::{
    handler,
    web::{Json, Path},
};

#[handler]
pub async fn get(Path(_path): Path<String>) -> Json<Option<entity::ship_ticket_bill::Model>> {
    let ship_ticket_bill = entity::ship_ticket_bill::get().await;
    Json(ship_ticket_bill)
}
