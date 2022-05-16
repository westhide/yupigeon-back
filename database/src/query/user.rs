use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::entity::{
    role, token,
    user::{Column, Entity, Link2Role, Model},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    user: Model,
    token_info: Option<token::Model>,
    roles: Vec<role::Model>,
}

pub async fn user(username: String, password: String) -> Result<UserInfo, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    let user_relate_token = Entity::find()
        .find_also_related(token::Entity)
        .filter(Column::Username.eq(username))
        .filter(Column::Password.eq(password))
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("用户名或密码错误".into()))?;

    let (user, token_info) = user_relate_token;

    let roles = user.find_linked(Link2Role).all(&txn).await?;

    Ok(UserInfo {
        user,
        token_info,
        roles,
    })
}
