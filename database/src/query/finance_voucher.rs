use sea_orm::entity::prelude::*;

use crate::entity::finance_voucher_template::{Column, Entity, Model};

pub async fn finance_voucher_template(code: &str) -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find().filter(Column::Code.eq(code)).all(&txn).await
}
