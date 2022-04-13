use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "finance_link_subsidiary_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    subsidiary_group_id: i32,
    subsidiary_account_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::finance_subsidiary_group::Entity",
        from = "Column::SubsidiaryGroupId",
        to = "super::finance_subsidiary_group::Column::Id"
    )]
    FinanceSubsidiaryGroup,
    #[sea_orm(
        belongs_to = "super::finance_subsidiary_account::Entity",
        from = "Column::SubsidiaryAccountId",
        to = "super::finance_subsidiary_account::Column::Id"
    )]
    FinanceSubsidiaryAccount,
}

impl ActiveModelBehavior for ActiveModel {}