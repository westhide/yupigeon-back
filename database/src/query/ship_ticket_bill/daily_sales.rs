use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct DailySales {
    date: Date,
    times: i64,
    sales: Option<Decimal>,
}

pub async fn daily_sales(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailySales>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let sql = r#"
        WITH td AS (
            SELECT
            ticket_status,
            departure_datetime,
            ticket_price
            FROM ticket_bill
            WHERE departure_datetime >=? AND departure_datetime <= ?
            )

        SELECT date(departure_datetime) date,count(1) times,sum(ticket_price) sales FROM td
        WHERE ticket_status IN ('一检','二检','出票成功')
        GROUP BY date
        ORDER BY date
        ;
    "#;

    database
        .find_by_sql_and_values(sql, vec![datetime_from.into(), datetime_end.into()])
        .await
}
