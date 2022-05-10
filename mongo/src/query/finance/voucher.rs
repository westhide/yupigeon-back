use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::account::{find_finance_account_info, FinanceAccountInfo};
use crate::{
    collection::{FinanceVoucherTemplate, OrganizationCompany},
    common::{CollectionTrait, DBRefTrait},
    error::{MongoErr, Result},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTemplateInfo {
    #[serde(flatten)]
    template: FinanceVoucherTemplate,
    debit_account_info: FinanceAccountInfo,
    credit_account_info: FinanceAccountInfo,
    organization_company: OrganizationCompany,
}
pub async fn voucher_template_info(code: &str) -> Result<VoucherTemplateInfo> {
    let template = FinanceVoucherTemplate::collection()
        .find_one(doc! {"code":code}, None)
        .await?
        .ok_or_else(|| MongoErr::not_found("FinanceVoucherTemplate"))?;

    let debit_ref = &template.debit_finance_account_ref;
    let credit_ref = &template.debit_finance_account_ref;
    let company_ref = &template.organization_company_ref;

    let debit_account_info = find_finance_account_info(doc! {"_id":debit_ref._id}, None).await?;
    let credit_account_info = find_finance_account_info(doc! {"_id":credit_ref._id}, None).await?;
    let organization_company = company_ref
        .fetch()
        .await?
        .ok_or_else(|| MongoErr::not_found("OrganizationCompany"))?;

    Ok(VoucherTemplateInfo {
        template,
        debit_account_info,
        credit_account_info,
        organization_company,
    })
}
