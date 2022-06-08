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
    id: u32,
    #[sea_orm(unique)]
    username: String,
    #[serde(skip_deserializing, skip_serializing)]
    password: String,
}

#[derive(Debug, EnumIter, DeriveRelation)]
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

#[derive(Debug)]
pub struct Link2Role;

use super::link_user2role;

impl Linked for Link2Role {
    type FromEntity = Entity;
    type ToEntity = super::role::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            link_user2role::Relation::User.def().rev(),
            link_user2role::Relation::Role.def(),
        ]
    }
}
