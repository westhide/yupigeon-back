use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip)]
    id: u32,
    group: String,
    name: String,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
