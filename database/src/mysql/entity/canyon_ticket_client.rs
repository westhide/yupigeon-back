use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_ticket_client")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    #[sea_orm(unique)]
    name: String,
    r#type: String,
    online_payment_type: Option<String>,
    online_channel: Option<String>,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
