use futures::stream::TryStream;
use mongodb::{
    bson::{doc, to_bson, Document},
    error::Error,
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
    error::{MongoErr, Result},
    query::common::{find_all_by_collection, QueryTrait},
};

async fn update_items<T>() -> Result<FinanceAssistAccount>
where
    T: CollectionTrait,
    Cursor<T>: TryStream,
    Vec<T>: Extend<<Cursor<T> as TryStream>::Ok>,
    Error: From<<Cursor<T> as TryStream>::Error>,
{
    let assist_refs = T::find_all()
        .await?
        .iter()
        .map(|item| item.db_ref())
        .collect::<Vec<DBRef>>();

    let assist_refs_bson = to_bson(&assist_refs).map_err(Into::<Error>::into)?;

    FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"collectionName":T::collection_name()},
            doc! {"$set":{"assistRefs":assist_refs_bson}},
            None,
        )
        .await?
        .ok_or_else(|| MongoErr::message_error("FinanceAssistAccount Not Found"))
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
    #[serde(flatten)]
    assist_account: FinanceAssistAccount,
    items: Vec<AssistAccountItem>,
}

async fn find_assist_account_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<AssistAccountInfo> {
    let assist_account = FinanceAssistAccount::collection()
        .find_one(filter, options)
        .await?
        .ok_or_else(|| MongoErr::message_error("FinanceAssistAccount Not Found"))?;

    let collection_name = &assist_account.collection_name;
    let items = find_all_by_collection::<AssistAccountItem>(collection_name).await?;

    let assist_account_info = AssistAccountInfo {
        assist_account,
        items,
    };
    Ok(assist_account_info)
}

pub async fn assist_account_info(name: &str) -> Result<AssistAccountInfo> {
    find_assist_account_info(doc! {"name":name}, None).await
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistAccountGroupInfo {
    #[serde(flatten)]
    assist_account_group: FinanceAssistAccountGroup,
    items: Vec<AssistAccountInfo>,
}

pub async fn find_assist_account_group_info(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<AssistAccountGroupInfo> {
    let assist_account_group = FinanceAssistAccountGroup::collection()
        .find_one(filter, options)
        .await?
        .ok_or_else(|| MongoErr::message_error("FinanceAssistAccountGroup Not Found"))?;

    let mut assist_account_group_info = AssistAccountGroupInfo {
        assist_account_group,
        items: vec![],
    };

    for db_ref in &assist_account_group_info
        .assist_account_group
        .assist_account_refs
    {
        let assist_account_info = find_assist_account_info(doc! {"_id":db_ref._id}, None).await?;
        assist_account_group_info.items.push(assist_account_info);
    }

    Ok(assist_account_group_info)
}

pub async fn assist_account_group_info(code: &str) -> Result<AssistAccountGroupInfo> {
    find_assist_account_group_info(doc! {"code":code}, None).await
}
