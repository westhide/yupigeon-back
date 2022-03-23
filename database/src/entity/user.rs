use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    #[serde(skip_deserializing, skip_serializing)]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get(username: String, password: String) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(Column::Username.eq(username))
        .filter(Column::Password.eq(password))
        .one(get_db("default"))
        .await
}
