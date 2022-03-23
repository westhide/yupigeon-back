use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "ticket_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    id: i32,
    #[sea_orm(column_type = "BigInteger")]
    link_order_id: i64,
    channel_name: String,
    #[sea_orm(column_type = "TinyInteger")]
    serial_no: i8,
    ticket_status: String,
    line_name: String,
    create_time: DateTime,
    departure_datetime: DateTime,
    ship_name: String,
    ticket_type_name: String,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    ticket_price: Decimal,
    cabin_name: String,
    seat_memo: String,
    passenger_name: String,
    passenger_id_no: String,
    user_name: String,
    payment_method: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    pay_amount: Option<Decimal>,
    payment_time: Option<DateTime>,
    pay_id: Option<String>,
    #[sea_orm(column_type = "BigInteger")]
    ticket_id: i64,
    link_ticket_id: String,
    #[sea_orm(column_type = "BigInteger")]
    ticket_no: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get(datetime_from: DateTime, datetime_end: DateTime) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .filter(Column::DepartureDatetime.gte(datetime_from))
        .filter(Column::DepartureDatetime.lte(datetime_end))
        .all(get_db("laiu8"))
        .await
}
