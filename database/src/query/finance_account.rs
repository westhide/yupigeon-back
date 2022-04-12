use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::entity::{
    finance_account::{Column, Entity, Link2FinanceAccount, Model},
    finance_subsidiary_account as SubAccount,
};

#[derive(Serialize)]
pub struct FinanceAccountLink {
    finance_account: Model,
    subsidiary_account: Vec<SubAccount::Model>,
}

pub async fn finance_account(code: &str) -> Result<FinanceAccountLink, DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    let finance_account = Entity::find()
        .filter(Column::Code.eq(code))
        .one(&txn)
        .await?;

    match finance_account {
        Some(finance_account) => {
            let subsidiary_account = finance_account
                .find_linked(Link2FinanceAccount)
                .all(&txn)
                .await?;
            Ok(FinanceAccountLink {
                finance_account,
                subsidiary_account,
            })
        }
        None => Err(DbErr::RecordNotFound("RecordNotFound".into())),
    }
}
