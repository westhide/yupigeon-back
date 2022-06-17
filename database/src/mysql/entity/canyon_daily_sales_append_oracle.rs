use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_daily_sales_append_oracle")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    trade_date: Date,
    trade_channel: String,
    operator_name: String,
    pay_type_name: String,
    client_full_name: String,
    ticket_model_name: String,
    ticket_count: i32,
    ticket_model_price: Decimal,
    ticket_amount: Decimal,
    remark: Option<String>,
    is_append: i8,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
