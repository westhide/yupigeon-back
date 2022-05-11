use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "investment_real_estates")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    name: String,
    location: Option<String>,
    code: String,
    floor: Option<String>,
    leasable_area: Decimal,
    status: Option<String>,
    attachment: Option<String>,
    remark: Option<String>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
