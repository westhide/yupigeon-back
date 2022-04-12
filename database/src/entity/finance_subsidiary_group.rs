use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::finance_link_subsidiary_group as link;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "finance_subsidiary_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::finance_account::Entity")]
    FinanceAccount,
}

impl Related<super::finance_subsidiary_account::Entity> for Entity {
    fn to() -> RelationDef {
        link::Relation::FinanceSubsidiaryAccount.def()
    }

    fn via() -> Option<RelationDef> {
        Some(link::Relation::FinanceSubsidiaryGroup.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct Link2FinanceSubsidiaryGroup;

impl Linked for Link2FinanceSubsidiaryGroup {
    type FromEntity = Entity;
    type ToEntity = super::finance_subsidiary_account::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            link::Relation::FinanceSubsidiaryGroup.def().rev(),
            link::Relation::FinanceSubsidiaryAccount.def(),
        ]
    }
}
