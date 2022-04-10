use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Conductor {
    value: String,
}

pub async fn conductors() -> Result<Vec<Conductor>, DbErr> {
    let txn = crate::Database::new("laiu8").await?.txn;
    Conductor::find_by_statement(Statement::from_string(
        txn.get_database_backend(),
        r#"
            SELECT  DISTINCT user_name AS value
            FROM ticket_bill
            WHERE user_type='线下'
           "#
        .into(),
    ))
    .all(&txn)
    .await
}
