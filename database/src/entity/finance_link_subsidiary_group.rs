use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_link_subsidiary_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    subsidiary_group_id: u32,
    subsidiary_account_id: u32,
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
