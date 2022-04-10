use sea_orm::entity::prelude::*;

use crate::entity::{
    token,
    user::{Column, Entity, Link2User, Model},
};

pub async fn user(
    username: String,
    password: String,
) -> Result<Option<(Model, Option<token::Model>)>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    Entity::find()
        .find_also_linked(Link2User)
        .filter(Column::Username.eq(username))
        .filter(Column::Password.eq(password))
        .one(&txn)
        .await
}
