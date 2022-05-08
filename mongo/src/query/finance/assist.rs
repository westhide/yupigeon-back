use futures::stream::TryStream;
use mongodb::{
    bson::{doc, to_bson, Document},
    error::{Error, Result},
    options::FindOneOptions,
    Cursor,
};
use serde::{Deserialize, Serialize};

use crate::{
    collection::{
        FinanceAssistAccount, FinanceAssistAccountGroup, FinanceAssistChannel, FinanceAssistClient,
        FinanceAssistPayment, FinanceAssistProduct, FinanceAssistSupplier, FinanceAssistTool,
    },
    common::{CollectionTrait, DBRef},
    query::common::{find_all_by_collection, QueryTrait},
};

async fn update_items<T>() -> Result<Option<FinanceAssistAccount>>
where
    T: CollectionTrait,
    Cursor<T>: TryStream,
    Vec<T>: Extend<<Cursor<T> as TryStream>::Ok>,
    Error: From<<Cursor<T> as TryStream>::Error>,
{
    let db_refs = T::find_all()
        .await?
        .iter()
        .map(|item| item.db_ref())
        .collect::<Vec<DBRef>>();

    let items = to_bson(&db_refs)?;

    FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"collectionName":T::collection_name()},
            doc! {"$set":{"assistItems":items}},
            None,
        )
        .await
}

pub async fn update_assist_account_items() -> Result<Vec<impl Serialize>> {
    Ok(vec![
        update_items::<FinanceAssistClient>().await?,
        update_items::<FinanceAssistSupplier>().await?,
        update_items::<FinanceAssistProduct>().await?,
        update_items::<FinanceAssistChannel>().await?,
        update_items::<FinanceAssistTool>().await?,
        update_items::<FinanceAssistPayment>().await?,
    ])
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountItem {
    code: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountInfo {
    name: String,
    items: Vec<AssistAccountItem>,
}

async fn get_assist_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<Option<AssistAccountInfo>> {
    let assist_account = FinanceAssistAccount::collection()
        .find_one(filter, options)
        .await?;

    if let Some(assist_account) = assist_account {
        let collection_name = assist_account.collection_name;
        let items = find_all_by_collection::<AssistAccountItem>(&collection_name).await?;

        let assist_account_info = AssistAccountInfo {
            name: assist_account.name,
            items,
        };
        Ok(Some(assist_account_info))
    } else {
        Ok(None)
    }
}

pub async fn assist_account_info(name: &str) -> Result<Option<AssistAccountInfo>> {
    get_assist_account_info(doc! {"name":name}, None).await
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountGroupInfo {
    name: String,
    items: Vec<Option<AssistAccountInfo>>,
}

pub async fn get_assist_account_group_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<Option<AssistAccountGroupInfo>> {
    let assist_account_group = FinanceAssistAccountGroup::collection()
        .find_one(filter, options)
        .await?;

    if let Some(assist_account_group) = assist_account_group {
        let assist_account_items = assist_account_group.assist_account_items;

        let mut assist_account_infos = vec![];

        for db_ref in assist_account_items {
            let assist_account_info =
                get_assist_account_info(doc! {"_id":db_ref._id}, None).await?;
            assist_account_infos.push(assist_account_info);
        }

        let assist_account_group_info = AssistAccountGroupInfo {
            name: assist_account_group.name,
            items: assist_account_infos,
        };
        Ok(Some(assist_account_group_info))
    } else {
        Ok(None)
    }
}

pub async fn assist_account_group_info(code: &str) -> Result<Option<AssistAccountGroupInfo>> {
    get_assist_account_group_info(doc! {"code":code}, None).await
}
