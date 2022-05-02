use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_online_ticket_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    order_no: String,
    out_order_no: String,
    order_datetime: DateTime,
    visit_datetime: DateTime,
    ticket_type: String,
    ticket_price: Decimal,
    ticket_num: i32,
    ticket_amount: Decimal,
    payment_status: String,
    tourist_name: String,
    phone_number: String,
    tourist_id_no: String,
    client: String,
    conductor: String,
    check_in_datetime: DateTime,
    is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
