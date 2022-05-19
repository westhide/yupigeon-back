use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
};
use serde::{Deserialize, Serialize};

use crate::{
    collection::FinanceAssistAccount,
    common::CollectionTrait,
    error::{MongoErr, Result},
    query::common::find_all_by_collection,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountItem {
    code: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountInfo {
    #[serde(flatten)]
    pub assist_account: FinanceAssistAccount,
    pub items: Option<Vec<AssistAccountItem>>,
}

pub async fn find_assist_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
    is_simple: bool,
    not_found_error_message: &str,
) -> Result<AssistAccountInfo> {
    let assist_account = FinanceAssistAccount::collection()
        .find_one(filter, options)
        .await?
        .ok_or_else(|| MongoErr::message_error(not_found_error_message))?;

    let collection_name = &assist_account.collection_name;
    let items = if is_simple {
        None
    } else {
        Some(find_all_by_collection::<AssistAccountItem>(collection_name).await?)
    };

    let assist_account_info = AssistAccountInfo {
        assist_account,
        items,
    };
    Ok(assist_account_info)
}

pub async fn assist_account_info(name: &str) -> Result<AssistAccountInfo> {
    find_assist_account_info(
        doc! {"name":name},
        None,
        false,
        &format!("FinanceAssistAccount Not Found: name='{}'", name),
    )
    .await
}

pub async fn finance_assist(collection_name: &str) -> Result<Vec<AssistAccountItem>> {
    find_all_by_collection::<AssistAccountItem>(collection_name).await
}
