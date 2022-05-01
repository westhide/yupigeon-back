use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all(serialize = "camelCase"))]
#[sea_orm(table_name = "canyon_daily_sales_append")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    date: Date,
    channel: String,
    operator: String,
    payment_method: String,
    client: String,
    ticket_type: String,
    ticket_num: i32,
    ticket_price: Decimal,
    ticket_amount: Decimal,
    remark: Option<String>,
    #[sea_orm(default_value = true)]
    is_append: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ReplaceModel {
    pub id: Option<u32>,
    date: Date,
    channel: String,
    operator: String,
    payment_method: String,
    client: String,
    ticket_type: String,
    ticket_num: i32,
    ticket_price: Decimal,
    ticket_amount: Decimal,
    remark: Option<String>,
    is_append: bool,
}
