use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::init_database;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[serde(skip_deserializing)]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get_user() {
    let db = init_database().await;
    let user = Entity::find_by_id(1).one(&db).await.unwrap();
    println!("{:?}", user)
}
