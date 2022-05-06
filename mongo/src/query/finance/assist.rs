use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    error::Result,
};
use serde::Serialize;

use crate::{
    collection::{
        FinanceAssistAccount, FinanceAssistChannel, FinanceAssistClient, FinanceAssistPayment,
        FinanceAssistProduct, FinanceAssistSupplier, FinanceAssistTool,
    },
    query::common::{CollectionTrait, DBRef},
};

pub async fn update_assist_account_items() -> Result<Vec<impl Serialize>> {
    let client_db_refs = FinanceAssistClient::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistClient>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let client_items = to_bson(&client_db_refs)?;

    let client_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"客户"},
            doc! {"$set":{"assist_items":client_items}},
            None,
        )
        .await?;

    let supplier_db_refs = FinanceAssistSupplier::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistSupplier>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let supplier_items = to_bson(&supplier_db_refs)?;
    let supplier_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"供应商"},
            doc! {"$set":{"assist_items":supplier_items}},
            None,
        )
        .await?;

    let product_db_refs = FinanceAssistProduct::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistProduct>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let product_items = to_bson(&product_db_refs)?;
    let product_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"产品"},
            doc! {"$set":{"assist_items":product_items}},
            None,
        )
        .await?;

    let channel_db_refs = FinanceAssistChannel::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistChannel>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let channel_items = to_bson(&channel_db_refs)?;
    let channel_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"销售渠道"},
            doc! {"$set":{"assist_items":channel_items}},
            None,
        )
        .await?;

    let tool_db_refs = FinanceAssistTool::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistTool>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let tool_items = to_bson(&tool_db_refs)?;
    let tool_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"运营工具"},
            doc! {"$set":{"assist_items":tool_items}},
            None,
        )
        .await?;

    let payment_db_refs = FinanceAssistPayment::collection()
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<FinanceAssistPayment>>()
        .await?
        .iter()
        .map(|item| DBRef::new(item.get_collection_name(), item._id))
        .collect::<Vec<DBRef>>();

    let payment_items = to_bson(&payment_db_refs)?;
    let payment_updated = FinanceAssistAccount::collection()
        .find_one_and_update(
            doc! {"name":"收款方式"},
            doc! {"$set":{"assist_items":payment_items}},
            None,
        )
        .await?;

    Ok(vec![
        client_updated,
        supplier_updated,
        product_updated,
        channel_updated,
        tool_updated,
        payment_updated,
    ])
}
