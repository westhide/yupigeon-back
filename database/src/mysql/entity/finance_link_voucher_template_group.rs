use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_link_voucher_template_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    voucher_template_group_id: u32,
    voucher_template_id: u32,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::finance_voucher_template_group::Entity",
        from = "Column::VoucherTemplateGroupId",
        to = "super::finance_voucher_template_group::Column::Id"
    )]
    FinanceVoucherTemplateGroup,
    #[sea_orm(
        belongs_to = "super::finance_voucher_template::Entity",
        from = "Column::VoucherTemplateId",
        to = "super::finance_voucher_template::Column::Id"
    )]
    FinanceVoucherTemplate,
}
