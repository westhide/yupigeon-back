use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    value: String,
}

pub async fn operators() -> Result<Vec<Operator>, DbErr> {
    let database = crate::Database::new("default").await?;
    database
        .find_by_sql(
            r#"
                SELECT  DISTINCT operator value FROM canyon_offline_ticket_bill
                UNION
                SELECT  DISTINCT client value FROM canyon_online_ticket_bill
                ;
           "#,
        )
        .await
}
