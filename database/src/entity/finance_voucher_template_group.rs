use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::finance_link_voucher_template_group as link;

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_voucher_template_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    code: String,
    name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Debug)]
pub struct Link2FinanceVoucherTemplate;

impl Linked for Link2FinanceVoucherTemplate {
    type FromEntity = Entity;
    type ToEntity = super::finance_voucher_template::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            link::Relation::FinanceVoucherTemplateGroup.def().rev(),
            link::Relation::FinanceVoucherTemplate.def(),
        ]
    }
}
