use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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
pub enum Relation {
    #[sea_orm(has_one = "super::token::Entity")]
    Token,
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct Link2User;

impl Linked for Link2User {
    type FromEntity = Entity;
    type ToEntity = super::token::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Token.def()]
    }
}

pub async fn get(
    username: String,
    password: String,
) -> Result<Option<(Model, Option<super::token::Model>)>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find()
        .find_also_linked(Link2User)
        .filter(Column::Username.eq(username))
        .filter(Column::Password.eq(password))
        .one(&txn)
        .await
}
