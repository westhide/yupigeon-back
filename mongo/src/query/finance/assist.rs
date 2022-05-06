use futures::stream::{TryStream, TryStreamExt};
use mongodb::{
    bson::{doc, to_bson},
    error::{Error, Result},
    Cursor,
};
use serde::Serialize;

use crate::{
    collection::{
        FinanceAssistAccount, FinanceAssistChannel, FinanceAssistClient, FinanceAssistPayment,
        FinanceAssistProduct, FinanceAssistSupplier, FinanceAssistTool,
    },
    query::common::{CollectionTrait, DBRef, DBRefTrait},
};

async fn update_items<T>(name: &str) -> Result<Option<FinanceAssistAccount>>
where
    T: CollectionTrait,
    Cursor<T>: TryStream,
    Vec<T>: Extend<<Cursor<T> as TryStream>::Ok>,
    Error: From<<Cursor<T> as TryStream>::Error>,
{
    let db_refs = T::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<T>>()
        .await?
        .iter()
        .map(|item| item.db_ref())
        .collect::<Vec<DBRef>>();

    let items = to_bson(&db_refs)?;

    FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":name},
            doc! {"$set":{"assistItems":items}},
            None,
        )
        .await
}

pub async fn update_assist_account_items() -> Result<Vec<impl Serialize>> {
    Ok(vec![
        update_items::<FinanceAssistClient>("客户").await?,
        update_items::<FinanceAssistSupplier>("供应商").await?,
        update_items::<FinanceAssistProduct>("产品").await?,
        update_items::<FinanceAssistChannel>("销售渠道").await?,
        update_items::<FinanceAssistTool>("运营工具").await?,
        update_items::<FinanceAssistPayment>("收款方式").await?,
    ])
}

pub async fn assist_account() -> Result<Option<FinanceAssistChannel>> {
    let assist_account = FinanceAssistAccount::collection()
        .find_one(doc! {}, None)
        .await?
        .unwrap();
    assist_account.assist_items.unwrap()[0]
        .fetch::<FinanceAssistChannel>()
        .await
}
