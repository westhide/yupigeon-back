use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::entity::{
    token,
    user::{Column, Entity, Model},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRelated {
    user: Model,
    token: Option<token::Model>,
}

pub async fn user(username: String, password: String) -> Result<UserRelated, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    let related = Entity::find()
        .find_also_related(token::Entity)
        .filter(Column::Username.eq(username))
        .filter(Column::Password.eq(password))
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("RecordNotFound".into()))?;

    let (user, token) = related;
    Ok(UserRelated { user, token })
}
