use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::finance_link_subsidiary_group as link;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "finance_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub code: String,
    pub name: String,
    pub direction: i8,
    pub subsidiary_group_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::finance_subsidiary_group::Entity",
        from = "Column::SubsidiaryGroupId",
        to = "super::finance_subsidiary_group::Column::Id"
    )]
    FinanceSubsidiaryGroup,
}

impl Related<super::finance_subsidiary_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FinanceSubsidiaryGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct Link2FinanceAccount;

impl Linked for Link2FinanceAccount {
    type FromEntity = Entity;
    type ToEntity = super::finance_subsidiary_account::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::FinanceSubsidiaryGroup.def(),
            link::Relation::FinanceSubsidiaryGroup.def().rev(),
            link::Relation::FinanceSubsidiaryAccount.def(),
        ]
    }
}
