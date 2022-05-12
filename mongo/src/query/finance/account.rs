use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
};
use serde::{Deserialize, Serialize};

use super::assist::{find_assist_account_group_info, AssistAccountGroupInfo};
use crate::{
    collection::FinanceAccount,
    common::CollectionTrait,
    error::{MongoErr, Result},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccountInfo {
    #[serde(flatten)]
    pub finance_account: FinanceAccount,
    pub assist_account_group_info: Option<AssistAccountGroupInfo>,
}

pub async fn find_finance_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<FinanceAccountInfo> {
    let finance_account = FinanceAccount::collection()
        .find_one(filter, options)
        .await?
        .ok_or_else(|| MongoErr::message_error("FinanceAccount Not Found"))?;

    let finance_account_info = FinanceAccountInfo {
        assist_account_group_info: if let Some(db_ref) = &finance_account.assist_account_group_ref {
            Some(find_assist_account_group_info(doc! {"_id":db_ref._id}, None).await?)
        } else {
            None
        },
        finance_account,
    };

    Ok(finance_account_info)
}

pub async fn finance_account_info(code: &str) -> Result<FinanceAccountInfo> {
    find_finance_account_info(doc! {"code":code}, None).await
}
