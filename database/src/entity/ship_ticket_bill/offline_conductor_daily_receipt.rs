use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct OfflineConductorDailyReceipt {
    date: Date,
    user_name: String,
    amount: Option<Decimal>,
}

pub async fn offline_conductor_daily_receipt(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<OfflineConductorDailyReceipt>, DbErr> {
    let txn = crate::get_txn("laiu8").await?;
    OfflineConductorDailyReceipt::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
            SELECT
                DATE( create_time ) date,
                user_name,
                SUM( pay_amount ) amount
            FROM ticket_bill
            WHERE
                channel_name NOT IN ('来游吧','驻岛订票','内部订票')
                AND pay_amount IS NOT NULL
                AND create_time >=? AND create_time <= ?
            GROUP BY date,user_name
            ORDER BY date DESC,user_name
        ;
        "#,
        vec![datetime_from.into(), datetime_end.into()],
    ))
    .all(&txn)
    .await
}
