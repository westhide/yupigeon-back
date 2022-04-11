use sea_orm::entity::prelude::*;

use crate::entity::{
    finance_account::{Column, Entity, Link2FinanceAccount, Model},
    finance_subsidiary_account as SubAccount,
};

pub async fn finance_account(code: &str) -> Result<(Model, Vec<SubAccount::Model>), DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    let finance_account = Entity::find()
        .filter(Column::Code.eq(code))
        .one(&txn)
        .await?;

    if let Some(finance_account) = finance_account {
        let subsidiary_account = finance_account
            .find_linked(Link2FinanceAccount)
            .all(&txn)
            .await?;
        Ok((finance_account, subsidiary_account))
    } else {
        Err(DbErr::RecordNotFound("RecordNotFound".into()))
    }
}
