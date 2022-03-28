use sea_orm::entity::prelude::*;

use crate::get_txn;

pub async fn get(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<super::Model>, DbErr> {
    let txn = get_txn("laiu8").await?;
    super::Entity::find()
        .filter(super::Column::DepartureDatetime.gte(datetime_from))
        .filter(super::Column::DepartureDatetime.lte(datetime_end))
        .all(&txn)
        .await
}
