use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::mysql::entity::{
    finance_account::{Column, Entity, Link2FinanceSubsidiaryAccount, Model},
    finance_subsidiary_account as SubAccount,
};

pub async fn finance_accounts() -> Result<Vec<Model>, DbErr> {
    let txn = crate::mysql::Database::new("default").await?.txn;
    Entity::find().all(&txn).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccountInfo {
    #[serde(flatten)]
    finance_account: Model,
    subsidiary_accounts: Vec<SubAccount::Model>,
}

pub async fn finance_account_info(code: &str) -> Result<FinanceAccountInfo, DbErr> {
    let txn = crate::mysql::Database::new("default").await?.txn;

    let finance_account = Entity::find()
        .filter(Column::Code.eq(code))
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("RecordNotFound".into()))?;

    let subsidiary_accounts = finance_account
        .find_linked(Link2FinanceSubsidiaryAccount)
        .all(&txn)
        .await?;

    Ok(FinanceAccountInfo {
        finance_account,
        subsidiary_accounts,
    })
}
