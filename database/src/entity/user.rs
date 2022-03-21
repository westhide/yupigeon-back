use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get() -> Option<Model> {
    Entity::find_by_id(1).one(get_db("default")).await.unwrap()
}
