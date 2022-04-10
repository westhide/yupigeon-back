use sea_orm::entity::prelude::*;

use crate::entity::ship_ticket_bill::{Column, Entity, Model};

pub async fn bill(datetime_from: DateTime, datetime_end: DateTime) -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("laiu8").await?.txn;
    Entity::find()
        .filter(Column::DepartureDatetime.gte(datetime_from))
        .filter(Column::DepartureDatetime.lte(datetime_end))
        .all(&txn)
        .await
}
