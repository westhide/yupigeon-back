use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::{
    account::{find_finance_account_info, FinanceAccountInfo},
    assist::{AssistAccountInfo, AssistAccountItem},
};
use crate::mongo::{
    collection::{
        finance_voucher_template::TemplateBase, FinanceAccount, FinanceVoucherTemplate,
        OrganizationCompany,
    },
    common::CollectionTrait,
    error::{MongoErr, Result},
    query::common::DBRefTrait,
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
pub async fn voucher_template_info(code: &str, is_simple: bool) -> Result<VoucherTemplateInfo> {
    let template = FinanceVoucherTemplate::collection()?
        .find_one(doc! {"code":code}, None)
        .await?
        .ok_or_else(|| {
            MongoErr::message_error(&format!(
                "FinanceVoucherTemplate Not Found: code='{}'",
                code
            ))
        })?;

    let FinanceVoucherTemplate {
        debit_finance_account_ref: debit_ref,
        credit_finance_account_ref: credit_ref,
        organization_company_ref: company_ref,
        ..
    } = &template;

    let debit_account_info = find_finance_account_info(
        doc! {"_id":debit_ref.ref_id},
        None,
        is_simple,
        &format!("FinanceAccount Not Found: _id='{}'", debit_ref.ref_id),
    )
    .await?;
    let credit_account_info = find_finance_account_info(
        doc! {"_id":credit_ref.ref_id},
        None,
        is_simple,
        &format!("FinanceAccount Not Found: _id='{}'", credit_ref.ref_id),
    )
    .await?;
    let organization_company = company_ref.fetch().await?;

    Ok(VoucherTemplateInfo {
        template,
        debit_account_info,
        credit_account_info,
        organization_company,
    })
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KingdeeAssistAccount {
    code: String,
    name: String,
    collection_name: String,
    items: Option<Vec<AssistAccountItem>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KingdeeAccount {
    finance_account_code: String,
    finance_account_name: String,
    assist_accounts: Vec<KingdeeAssistAccount>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KingdeeCloudVoucherTemplate {
    #[serde(flatten)]
    template_base: TemplateBase,
    company_finance_code: String,
    company_name: String,
    debit_account: KingdeeAccount,
    credit_account: KingdeeAccount,
}

fn parse_kingdee_account(
    finance_account: FinanceAccount,
    assist_infos: Vec<AssistAccountInfo>,
) -> KingdeeAccount {
    KingdeeAccount {
        finance_account_code: finance_account.code,
        finance_account_name: finance_account.name,
        assist_accounts: assist_infos
            .iter()
            .map(
                |AssistAccountInfo {
                     assist_account,
                     items,
                 }| KingdeeAssistAccount {
                    code: assist_account.code.to_owned(),
                    name: assist_account.name.to_owned(),
                    collection_name: assist_account.collection_name.to_owned(),
                    items: items.to_owned(),
                },
            )
            .collect(),
    }
}

pub async fn kingdee_cloud_voucher_template(
    code: &str,
    is_simple: bool,
) -> Result<KingdeeCloudVoucherTemplate> {
    let template_info = voucher_template_info(code, is_simple).await?;
    let VoucherTemplateInfo {
        template: FinanceVoucherTemplate { template_base, .. },
        organization_company:
            OrganizationCompany {
                finance_code: company_finance_code,
                name: company_name,
                ..
            },
        debit_account_info:
            FinanceAccountInfo {
                finance_account: debit_finance_account,
                assist_account_infos: debit_assist_infos,
            },
        credit_account_info:
            FinanceAccountInfo {
                finance_account: credit_finance_account,
                assist_account_infos: credit_assist_infos,
            },
    } = template_info;

    let voucher_template = KingdeeCloudVoucherTemplate {
        template_base,
        company_finance_code,
        company_name,
        debit_account: parse_kingdee_account(debit_finance_account, debit_assist_infos),
        credit_account: parse_kingdee_account(credit_finance_account, credit_assist_infos),
    };

    Ok(voucher_template)
}
