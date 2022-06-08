use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "mapper_domain_value")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    domain: String,
    r#type: String,
    from_value: Option<String>,
    to_value: Option<String>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
