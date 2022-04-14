use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_subsidiary_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    code: String,
    name: String,
    field: Option<String>,
    items: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
