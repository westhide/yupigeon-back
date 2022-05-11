use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "ship_ticket_refund_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    id: i32,
    tb_id: i64,
    #[sea_orm(column_type = "BigInteger")]
    ticket_id: i64,
    link_ticket_id: String,
    #[sea_orm(column_type = "BigInteger")]
    ticket_no: i64,
    refund_type: String,
    channel_name: Option<String>,
    user_type: Option<String>,
    user_name: Option<String>,
    refund_method: Option<String>,
    refund_finish_time: Option<DateTime>,
    refund_id: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    refund_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    fee: Decimal,
    #[sea_orm(column_type = "BigInteger")]
    order_id: i64,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
