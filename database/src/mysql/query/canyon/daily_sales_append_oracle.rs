use sea_orm::entity::prelude::*;

use crate::mysql::entity::canyon_daily_sales_append_oracle::Model;

pub async fn daily_sales_append_oracle(
    datetime_from: &str,
    datetime_end: &str,
    condition: &str,
) -> Result<Vec<Model>, DbErr> {
    let database = crate::mysql::Database::new("default").await?;

    let sql = format!(
        "
        SELECT
            *
        FROM
            canyon_daily_sales_append_oracle
        WHERE
            is_append = TRUE
            AND trade_date BETWEEN DATE(?) AND DATE(?)
            {}
            ;
        ",
        condition
    );

    database
        .find_by_sql_and_values(&sql, vec![datetime_from.into(), datetime_end.into()])
        .await
}
