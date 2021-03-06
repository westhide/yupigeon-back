use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_subsidiary_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    name: String,
}

#[derive(Debug, EnumIter, DeriveRelation)]
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

#[derive(Debug)]
pub struct Link2FinanceSubsidiaryAccount;

use super::finance_link_subsidiary_group as link;

impl Linked for Link2FinanceSubsidiaryAccount {
    type FromEntity = Entity;
    type ToEntity = super::finance_subsidiary_account::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            link::Relation::FinanceSubsidiaryGroup.def().rev(),
            link::Relation::FinanceSubsidiaryAccount.def(),
        ]
    }
}
