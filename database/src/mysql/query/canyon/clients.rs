use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    value: String,
}

pub async fn clients() -> Result<Vec<Client>, DbErr> {
    let database = crate::mysql::Database::new("default").await?;
    database
        .find_by_sql(
            r#"
                SELECT  DISTINCT client value FROM canyon_offline_ticket_bill
                UNION
                SELECT  DISTINCT client value FROM canyon_online_ticket_bill
                UNION
                SELECT  DISTINCT name value FROM canyon_ticket_client
                ;
           "#,
        )
        .await
}
