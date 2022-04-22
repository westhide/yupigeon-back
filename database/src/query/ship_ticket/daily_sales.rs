use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySales {
    date: Date,
    times: i64,
    sales: Decimal,
}

pub async fn daily_sales(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailySales>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let sql = r#"
        WITH tbd AS
        (
            SELECT  DISTINCT DATE(departure_datetime) date
            FROM ticket_bill
            WHERE departure_datetime BETWEEN ? AND ?
        ) , tbs AS
        (
            SELECT  DATE(departure_datetime) date
                ,COUNT(1) times
                ,SUM(ticket_price) sales
            FROM ticket_bill
            WHERE ticket_status IN ('一检', '二检', '出票成功')
            AND departure_datetime BETWEEN ? AND ?
            GROUP BY  date
        )
        SELECT  tbd.date
            ,IFNULL(tbs.times,0) times
            ,IFNULL(tbs.sales,0) sales
        FROM tbd
        LEFT JOIN tbs
        ON tbd.date=tbs.date
        ORDER BY date
        ;
    "#;

    database
        .find_by_sql_and_values(
            sql,
            vec![
                datetime_from.into(),
                datetime_end.into(),
                datetime_from.into(),
                datetime_end.into(),
            ],
        )
        .await
}
