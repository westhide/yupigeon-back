use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Conductor {
    value: String,
}

pub async fn conductors() -> Result<Vec<Conductor>, DbErr> {
    let database = crate::mysql::Database::new("laiu8").await?;

    database
        .find_by_sql(
            r#"
                SELECT  DISTINCT user_name AS value
                FROM ticket_bill
                WHERE user_type='线下'
                ;
           "#,
        )
        .await
}
