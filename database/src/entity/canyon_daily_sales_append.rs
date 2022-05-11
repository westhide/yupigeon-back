use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_daily_sales_append")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    is_append: i8,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
