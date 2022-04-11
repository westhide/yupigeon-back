use sea_orm::entity::prelude::*;

use crate::entity::finance_account::{Column, Entity, Model};

pub async fn finance_account(code: &str) -> Result<Option<Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find().filter(Column::Code.eq(code)).one(&txn).await
}
