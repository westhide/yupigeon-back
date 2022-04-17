use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    #[sea_orm(unique)]
    username: String,
    #[sea_orm(unique)]
    #[serde(skip_deserializing, skip_serializing)]
    password: String,
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

#[derive(Debug)]
pub struct Link2Token;

impl Linked for Link2Token {
    type FromEntity = Entity;
    type ToEntity = super::token::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Token.def()]
    }
}
