use mongodb::{bson::doc, error::Result};
use serde::{Deserialize, Serialize};

use super::account::{find_finance_account_info, FinanceAccountInfo};
use crate::{
    collection::{FinanceVoucherTemplate, OrganizationCompany},
    common::{CollectionTrait, DBRefTrait},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherTemplateInfo {
    #[serde(flatten)]
    template: FinanceVoucherTemplate,
    debit_account_info: Option<FinanceAccountInfo>,
    credit_account_info: Option<FinanceAccountInfo>,
    organization_company: Option<OrganizationCompany>,
}
pub async fn voucher_template_info(code: &str) -> Result<Option<VoucherTemplateInfo>> {
    let template = FinanceVoucherTemplate::collection()
        .find_one(doc! {"code":code}, None)
        .await?;

    if let Some(template) = template {
        let mut template_info = VoucherTemplateInfo {
            template,
            debit_account_info: None,
            credit_account_info: None,
            organization_company: None,
        };

        let debit_ref = &template_info.template.debit_finance_account_ref;
        let credit_ref = &template_info.template.debit_finance_account_ref;
        let company_ref = &template_info.template.organization_company_ref;

        template_info.debit_account_info =
            find_finance_account_info(doc! {"_id":debit_ref._id}, None).await?;
        template_info.credit_account_info =
            find_finance_account_info(doc! {"_id":credit_ref._id}, None).await?;
        template_info.organization_company = company_ref.fetch().await?;

        Ok(Some(template_info))
    } else {
        Ok(None)
    }
}
