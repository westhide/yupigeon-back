use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
};
use serde::{Deserialize, Serialize};

use super::assist::{find_assist_account_info, AssistAccountInfo};
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
    pub assist_account_infos: Vec<AssistAccountInfo>,
}

pub async fn find_finance_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
    is_simple: bool,
    not_found_error_message: &str,
) -> Result<FinanceAccountInfo> {
    let finance_account = FinanceAccount::collection()?
        .find_one(filter, options)
        .await?
        .ok_or_else(|| MongoErr::message_error(not_found_error_message))?;

    let mut assist_account_infos = vec![];
    if let Some(assist_account_refs) = &finance_account.assist_account_refs {
        for db_ref in assist_account_refs {
            let assist_account_info = find_assist_account_info(
                doc! {"_id":db_ref.ref_id},
                None,
                is_simple,
                &format!("FinanceAssistAccount Not Found: _id='{}'", db_ref.ref_id),
            )
            .await?;
            assist_account_infos.push(assist_account_info);
        }
    };
    let finance_account_info = FinanceAccountInfo {
        finance_account,
        assist_account_infos,
    };

    Ok(finance_account_info)
}

pub async fn finance_account_info(code: &str) -> Result<FinanceAccountInfo> {
    find_finance_account_info(
        doc! {"code":code},
        None,
        false,
        &format!("FinanceAccount Not Found: code='{}'", code),
    )
    .await
}
