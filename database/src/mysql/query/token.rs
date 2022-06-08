use sea_orm::entity::prelude::*;

use crate::mysql::entity::token::{Column, Entity, Model};

pub async fn token(user_id: u32) -> Result<Option<Model>, DbErr> {
    let txn = crate::mysql::Database::new("default").await?.txn;
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .one(&txn)
        .await
}
