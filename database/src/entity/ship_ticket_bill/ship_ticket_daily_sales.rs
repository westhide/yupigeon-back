use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
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
    let txn = crate::get_txn("laiu8").await?;
    DailySales::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
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
        "#,
        vec![datetime_from.into(), datetime_end.into()],
    ))
    .all(&txn)
    .await
}
