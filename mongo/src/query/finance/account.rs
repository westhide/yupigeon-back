use mongodb::{bson::doc, error::Result};
use serde::{Deserialize, Serialize};

use super::assist::{get_assist_account_group_info, AssistAccountGroupInfo};
use crate::{collection::FinanceAccount, common::CollectionTrait};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccountInfo {
    code: String,
    name: String,
    direction: String,
    assist_account_group_info: Option<AssistAccountGroupInfo>,
}

impl FinanceAccountInfo {
    fn new(
        finance_account: FinanceAccount,
        assist_account_group_info: Option<AssistAccountGroupInfo>,
    ) -> Result<Option<FinanceAccountInfo>> {
        Ok(Some(FinanceAccountInfo {
            code: finance_account.code,
            name: finance_account.name,
            direction: finance_account.direction,
            assist_account_group_info,
        }))
    }
}

pub async fn finance_account_info(code: &str) -> Result<Option<FinanceAccountInfo>> {
    let finance_account = FinanceAccount::collection()
        .find_one(doc! {"code":code}, None)
        .await?;

    if let Some(finance_account) = finance_account {
        match &finance_account.assist_account_group {
            Some(db_ref) => {
                let assist_account_group_info =
                    get_assist_account_group_info(doc! {"_id":db_ref._id}, None).await?;

                FinanceAccountInfo::new(finance_account, assist_account_group_info)
            }
            None => FinanceAccountInfo::new(finance_account, None),
        }
    } else {
        Ok(None)
    }
}
