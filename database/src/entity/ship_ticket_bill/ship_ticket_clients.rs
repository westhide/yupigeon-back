use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Clients {
    value: String,
}

pub async fn clients() -> Result<Vec<Clients>, DbErr> {
    let txn = crate::Database::new("laiu8").await?.txn;
    Clients::find_by_statement(Statement::from_string(
        txn.get_database_backend(),
        r#"
            SELECT  DISTINCT u8_vip_pact as value
            FROM ticket_bill
            WHERE IFNULL(u8_vip_pact,'')!=''
           "#
        .into(),
    ))
    .all(&txn)
    .await
}
