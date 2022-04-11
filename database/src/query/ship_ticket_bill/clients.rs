use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Client {
    value: String,
}

pub async fn clients() -> Result<Vec<Client>, DbErr> {
    let database = crate::Database::new("laiu8").await?;
    database
        .find_by_sql(
            r#"
                SELECT  DISTINCT u8_vip_pact as value
                FROM ticket_bill
                WHERE IFNULL(u8_vip_pact,'')!=''
           "#,
        )
        .await
}
