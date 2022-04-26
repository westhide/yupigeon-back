use sea_orm::{entity::prelude::*, EntityTrait, Set};
use serde::Serialize;

use crate::entity::{
    finance_subsidiary_account as SubAccount, finance_subsidiary_business as SubBusiness,
    finance_subsidiary_client as SubClient, finance_subsidiary_conductor as SubConductor,
    finance_subsidiary_group as SubGroup, finance_subsidiary_receipt_type as SubReceiptType,
    finance_subsidiary_ship as SubShip, finance_subsidiary_ship_line as SubShipLine,
    finance_subsidiary_tax as SubTax,
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

async fn update_items<E>(code: &str) -> Result<SubAccount::Model, DbErr>
where
    E: EntityTrait,
    E::Model: Serialize,
{
    let txn = crate::Database::new("default").await?.txn;
    let items = E::find().all(&txn).await?;
    let items_json = serde_json::json!(items);

    let mut sub_account: SubAccount::ActiveModel =
        match find_subsidiary_account_by_code(code).await? {
            Some(model) => model.into(),
            None => return Err(DbErr::RecordNotFound("RecordNotFound".into())),
        };

    sub_account.items = Set(items_json.into());

    let result = sub_account.update(&txn).await?;

    txn.commit().await?;
    Ok(result)
}

pub async fn subsidiary_clients() -> Result<Vec<SubClient::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    SubClient::Entity::find().all(&txn).await
}

pub async fn update_subsidiary_account_items() -> Result<Vec<SubAccount::Model>, DbErr> {
    Ok(vec![
        update_items::<SubClient::Entity>("00001").await?,
        update_items::<SubShipLine::Entity>("00028").await?,
        update_items::<SubShip::Entity>("00029").await?,
        update_items::<SubBusiness::Entity>("00031").await?,
        update_items::<SubReceiptType::Entity>("00044").await?,
        update_items::<SubConductor::Entity>("00058").await?,
        update_items::<SubTax::Entity>("00074").await?,
    ])
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryGroupInfo {
    #[serde(flatten)]
    subsidiary_group: SubGroup::Model,
    subsidiary_account: Vec<SubAccount::Model>,
}

pub async fn subsidiary_group_info(id: u32) -> Result<SubsidiaryGroupInfo, DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    let subsidiary_group = SubGroup::Entity::find_by_id(id)
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("RecordNotFound".into()))?;

    let subsidiary_account = subsidiary_group
        .find_linked(SubGroup::Link2FinanceSubsidiaryAccount)
        .all(&txn)
        .await?;

    Ok(SubsidiaryGroupInfo {
        subsidiary_group,
        subsidiary_account,
    })
}
