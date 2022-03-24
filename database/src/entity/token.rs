use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "token")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get(user_id: u32) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .one(get_db("default"))
        .await
}
