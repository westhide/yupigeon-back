use sea_orm::{entity::prelude::*, EntityTrait, Set};
use serde::Serialize;

use crate::entity::{
    finance_subsidiary_account as SubAccount, finance_subsidiary_client as SubClient,
    finance_subsidiary_conductor as SubConductor,
    finance_subsidiary_receipt_type as SubReceiptType,
};

pub async fn find_subsidiary_account_by_code(
    code: &str,
) -> Result<Option<SubAccount::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    SubAccount::Entity::find()
        .filter(SubAccount::Column::Code.eq(code))
        .one(&txn)
        .await
}

async fn update_subsidiary_account_items<E>(code: &str) -> Result<SubAccount::Model, DbErr>
where
    E: EntityTrait,
    E::Model: Serialize,
{
    let txn = crate::Database::new("default").await?.txn;
    let items = E::find().all(&txn).await?;
    let items_json = serde_json::json!(items);

    let mut sub_account: SubAccount::ActiveModel =
        find_subsidiary_account_by_code(code).await?.unwrap().into();

    sub_account.items = Set(items_json.into());

    let result = sub_account.update(&txn).await?;

    txn.commit().await?;
    Ok(result)
}

pub async fn subsidiary_clients() -> Result<Vec<SubClient::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    SubClient::Entity::find().all(&txn).await
}

pub async fn update_items() -> Result<Vec<SubAccount::Model>, DbErr> {
    let sub_client = update_subsidiary_account_items::<SubClient::Entity>("00001").await?;
    let sub_receipt_type =
        update_subsidiary_account_items::<SubReceiptType::Entity>("00044").await?;
    let sub_conductor = update_subsidiary_account_items::<SubConductor::Entity>("00058").await?;

    let result = vec![sub_client, sub_receipt_type, sub_conductor];
    Ok(result)
}
