use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct OfflineConductorDailyReceipt {
    date: Date,
    user_name: String,
    receipt: Option<Decimal>,
    sum_pay_amount: Option<Decimal>,
    sum_fee: Option<Decimal>,
    sum_refund_amount: Option<Decimal>,
}

pub async fn offline_conductor_daily_receipt(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<OfflineConductorDailyReceipt>, DbErr> {
    let txn = crate::get_txn("laiu8").await?;
    OfflineConductorDailyReceipt::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
            WITH offr AS (
                SELECT
                    DATE( refund_finish_time ) date,
                    user_name,
                    SUM(IF(refund_amount>0,refund_amount+fee,0)) sum_refund_amount,
                    SUM(fee) sum_fee
                FROM ship_ticket_refund_bill
                WHERE
                        user_type='线下'
                        AND refund_finish_time >=? AND refund_finish_time <= ?
                GROUP BY date,user_name
            )
            ,offp AS (
                SELECT
                        DATE( create_time ) date,
                        user_name,
                        SUM( pay_amount ) sum_pay_amount
                FROM ticket_bill
                WHERE
                        user_type='线下'
                        AND pay_amount IS NOT NULL
                        AND create_time >=? AND create_time <= ?
                GROUP BY date,user_name
            )
            ,off AS (
            SELECT
                offp.date,
                offp.user_name,
                offp.sum_pay_amount,
                offr.sum_fee,
                offr.sum_refund_amount
            FROM offp LEFT JOIN offr ON offp.date=offr.date AND offp.user_name=offr.user_name
            UNION
            SELECT
                offr.date,
                offr.user_name,
                offp.sum_pay_amount,
                offr.sum_fee,
                offr.sum_refund_amount
            FROM offp RIGHT JOIN offr ON offp.date=offr.date AND offp.user_name=offr.user_name
            )
            SELECT
                off.date,
                off.user_name,
                IFNULL(off.sum_pay_amount,0) + IFNULL(off.sum_fee,0) receipt,
                off.sum_pay_amount,
                off.sum_fee,
                off.sum_refund_amount
            FROM
                off
            WHERE off.sum_pay_amount>0 OR off.sum_fee>0 OR off.sum_refund_amount>0
            ORDER BY off.date DESC,off.user_name
        ;
        "#,
        vec![
            datetime_from.into(),
            datetime_end.into(),
            datetime_from.into(),
            datetime_end.into(),
        ],
    ))
    .all(&txn)
    .await
}
