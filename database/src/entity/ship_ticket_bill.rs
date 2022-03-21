use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "ticket_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    // pub link_order_id: i32,
    channel_name: String,
    serial_no: i32,
    ticket_status: String,
    line_name: String,
    // departure_datetime ,
    ship_name: String,
    ticket_type_name: String,
    // ticket_price ,
    cabin_name: String,
    seat_memo: String,
    passenger_name: String,
    passenger_id_no: String,
    user_name: String,
    payment_method: String,
    // pay_amount ,
    // payment_time ,
    pay_id: String,
    // ticket_id ,
    // link_ticket_id,
    // ticket_no
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get() -> Option<Model> {
    Entity::find_by_id(1).one(get_db("laiu8")).await.unwrap()
}
