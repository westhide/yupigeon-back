use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_offline_ticket_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    trade_type: String,
    operator: String,
    trade_no: String,
    group_no: Option<String>,
    upload_status: String,
    barcode: String,
    trade_time: DateTime,
    client: String,
    product_type: String,
    ticket_type: String,
    ticket_kind: String,
    ticket_price: Decimal,
    ticket_num: i32,
    ticket_amount: Decimal,
    payment_method: String,
    remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
