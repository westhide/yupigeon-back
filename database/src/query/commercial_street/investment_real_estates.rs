use sea_orm::entity::prelude::*;

use crate::entity::investment_real_estates::{Entity, Model};

pub async fn bill() -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find().all(&txn).await
}
