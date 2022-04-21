use sea_orm::entity::prelude::*;
use serde::Serialize;

use super::account::{finance_account_info, FinanceAccountInfo};
use crate::entity::{
    finance_voucher_template as VoucherTemplate,
    finance_voucher_template_group as VoucherTemplateGroup,
};

pub async fn voucher_template(code: &str) -> Result<Option<VoucherTemplate::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    VoucherTemplate::Entity::find()
        .filter(VoucherTemplate::Column::Code.eq(code))
        .one(&txn)
        .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTemplateInfo {
    #[serde(flatten)]
    template: VoucherTemplate::Model,
    debit_info: FinanceAccountInfo,
    credit_info: FinanceAccountInfo,
}

pub async fn voucher_template_info(code: &str) -> Result<VoucherTemplateInfo, DbErr> {
    let template = voucher_template(code)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("RecordNotFound".into()))?;

    let debit_info = finance_account_info(&template.debit_finance_account_code).await?;
    let credit_info = finance_account_info(&template.credit_finance_account_code).await?;

    Ok(VoucherTemplateInfo {
        template,
        debit_info,
        credit_info,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTemplateGroupInfo {
    #[serde(flatten)]
    template_group: VoucherTemplateGroup::Model,
    template_infos: Vec<VoucherTemplateInfo>,
}

pub async fn voucher_template_group(code: &str) -> Result<VoucherTemplateGroupInfo, DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    let template_group = VoucherTemplateGroup::Entity::find()
        .filter(VoucherTemplateGroup::Column::Code.eq(code))
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("RecordNotFound".into()))?;

    let templates = template_group
        .find_linked(VoucherTemplateGroup::Link2FinanceVoucherTemplate)
        .all(&txn)
        .await?;

    let mut template_infos = vec![];
    for template in templates {
        let template_info = voucher_template_info(&template.code).await?;
        template_infos.push(template_info);
    }

    Ok(VoucherTemplateGroupInfo {
        template_group,
        template_infos,
    })
}
