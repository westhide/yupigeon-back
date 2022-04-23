use sea_orm::entity::prelude::*;

use crate::entity::mapper_domain_value::{Column, Entity, Model};

pub async fn domain_value(domain: &str, r#type: &str) -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find()
        .filter(Column::Domain.eq(domain))
        .filter(Column::Type.eq(r#type))
        .all(&txn)
        .await
}
