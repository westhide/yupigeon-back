use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::FindOneOptions,
};
use serde::{Deserialize, Serialize};

use super::assist::{find_assist_account_group_info, AssistAccountGroupInfo};
use crate::{collection::FinanceAccount, common::CollectionTrait};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccountInfo {
    #[serde(flatten)]
    finance_account: FinanceAccount,
    assist_account_group_info: Option<AssistAccountGroupInfo>,
}

pub async fn find_finance_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<Option<FinanceAccountInfo>> {
    let finance_account = FinanceAccount::collection()
        .find_one(filter, options)
        .await?;

    if let Some(finance_account) = finance_account {
        let mut finance_account_info = FinanceAccountInfo {
            finance_account,
            assist_account_group_info: None,
        };

        if let Some(db_ref) = &finance_account_info
            .finance_account
            .assist_account_group_ref
        {
            finance_account_info.assist_account_group_info =
                find_assist_account_group_info(doc! {"_id":db_ref._id}, None).await?;
        }

        Ok(Some(finance_account_info))
    } else {
        Ok(None)
    }
}

pub async fn finance_account_info(code: &str) -> Result<Option<FinanceAccountInfo>> {
    find_finance_account_info(doc! {"code":code}, None).await
}
