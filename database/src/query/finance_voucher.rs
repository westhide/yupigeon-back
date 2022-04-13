use sea_orm::entity::prelude::*;
use serde::Serialize;

use super::finance_account::{finance_account_info, FinanceAccountInfo};
use crate::entity::finance_voucher_template::{Column, Entity, Model};

pub async fn voucher_template(code: &str) -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find().filter(Column::Code.eq(code)).all(&txn).await
}

#[derive(Serialize)]
pub struct VoucherTemplateInfo {
    template: Model,
    finance_account_info: FinanceAccountInfo,
}

pub async fn voucher_template_info(code: &str) -> Result<Vec<VoucherTemplateInfo>, DbErr> {
    let voucher_templates = voucher_template(code).await?;

    let mut voucher_template_info_group = vec![];
    for template in voucher_templates {
        let finance_account_code = &template.finance_account_code;
        let finance_account_info = finance_account_info(finance_account_code).await?;
        let voucher_template_info = VoucherTemplateInfo {
            template,
            finance_account_info,
        };
        voucher_template_info_group.push(voucher_template_info);
    }

    Ok(voucher_template_info_group)
}
