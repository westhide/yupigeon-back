use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_statistics_times")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    trade_date: Date,
    tool_name: String,
    name: String,
    times: i32,
    is_deleted: i8,
    remark: Option<String>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
